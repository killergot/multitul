import asyncio
import json
import os
import time
import uuid
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, Optional
from urllib.parse import urlsplit

from websockets.asyncio.server import serve
from websockets.exceptions import ConnectionClosed


SERVER_HOST = os.getenv("SERVER_HOST", "0.0.0.0")
SERVER_PORT = int(os.getenv("SERVER_PORT", "8765"))
DASHBOARD_HOST = os.getenv("DASHBOARD_HOST", "0.0.0.0")
DASHBOARD_PORT = int(os.getenv("DASHBOARD_PORT", "8080"))

MAX_PLAYERS_PER_ROOM = 2
MAX_CHAT_HISTORY = 100
MAX_CHAT_MESSAGE_LEN = 500
MAX_WORD_LEN = 100

DASHBOARD_HTML = Path(__file__).with_name("dashboard.html").read_text(encoding="utf-8")
HTTP_STATUS_TEXT = {
    200: "OK",
    400: "Bad Request",
    404: "Not Found",
    405: "Method Not Allowed",
    500: "Internal Server Error",
}


@dataclass
class Player:
    id: str
    name: str
    websocket: object


@dataclass
class ChatMessage:
    sender_id: str
    sender_name: str
    text: str
    timestamp: float


@dataclass
class Room:
    id: str
    players: Dict[str, Player] = field(default_factory=dict)
    submissions: Dict[str, str] = field(default_factory=dict)
    round_number: int = 1
    finished: bool = False
    history: list = field(default_factory=list)
    chat_history: list[ChatMessage] = field(default_factory=list)
    updated_at: float = field(default_factory=time.time)

    def is_full(self) -> bool:
        return len(self.players) >= MAX_PLAYERS_PER_ROOM

    def player_names(self) -> list[str]:
        return [player.name for player in self.players.values()]

    def ready_count(self) -> int:
        return len(self.submissions)


rooms: Dict[str, Room] = {}
player_to_room: Dict[str, str] = {}


def touch_room(room: Room):
    room.updated_at = time.time()


async def send_json(ws, data: dict):
    await ws.send(json.dumps(data, ensure_ascii=False))


async def broadcast(room: Room, data: dict):
    disconnected = []

    for player in room.players.values():
        try:
            await send_json(player.websocket, data)
        except Exception:
            disconnected.append(player.id)

    for player_id in disconnected:
        await remove_player(player_id)


async def broadcast_room_state(room: Room):
    await broadcast(room, {
        "type": "room_state",
        "room_id": room.id,
        "players": room.player_names(),
        "ready_count": room.ready_count(),
        "total_players": len(room.players),
        "round": room.round_number,
        "finished": room.finished,
    })


def serialize_chat_message(message: ChatMessage) -> dict:
    return {
        "sender_id": message.sender_id,
        "sender_name": message.sender_name,
        "text": message.text,
        "timestamp": message.timestamp,
    }


def serialize_player(player: Player) -> dict:
    return {
        "id": player.id,
        "name": player.name,
    }


def serialize_room_history_entry(entry: dict) -> dict:
    return {
        "round": entry["round"],
        "words": dict(entry["words"]),
        "match": bool(entry["match"]),
    }


def serialize_room_snapshot(room: Room) -> dict:
    players = sorted(
        (serialize_player(player) for player in room.players.values()),
        key=lambda item: item["name"].lower(),
    )

    submissions = []
    for player_id, word in room.submissions.items():
        player = room.players.get(player_id)
        submissions.append({
            "player_id": player_id,
            "player_name": player.name if player is not None else player_id,
            "word": word,
        })

    submissions.sort(key=lambda item: item["player_name"].lower())

    return {
        "id": room.id,
        "player_count": len(room.players),
        "max_players": MAX_PLAYERS_PER_ROOM,
        "ready_count": room.ready_count(),
        "round_number": room.round_number,
        "finished": room.finished,
        "last_activity_at": room.updated_at,
        "players": players,
        "submissions": submissions,
        "history": [serialize_room_history_entry(entry) for entry in room.history],
        "chat_history": [serialize_chat_message(msg) for msg in room.chat_history],
    }


def build_dashboard_state() -> dict:
    room_items = sorted(rooms.values(), key=lambda item: item.id.lower())
    room_snapshots = [serialize_room_snapshot(room) for room in room_items]

    return {
        "generated_at": time.time(),
        "summary": {
            "room_count": len(room_snapshots),
            "player_count": sum(room["player_count"] for room in room_snapshots),
            "finished_room_count": sum(1 for room in room_snapshots if room["finished"]),
            "chat_message_count": sum(len(room["chat_history"]) for room in room_snapshots),
        },
        "rooms": room_snapshots,
    }


async def send_chat_history(player: Player, room: Room):
    await send_json(player.websocket, {
        "type": "chat_history",
        "room_id": room.id,
        "messages": [serialize_chat_message(msg) for msg in room.chat_history],
    })


async def add_system_message(room: Room, text: str):
    system_message = ChatMessage(
        sender_id="system",
        sender_name="System",
        text=text,
        timestamp=time.time(),
    )
    room.chat_history.append(system_message)
    room.chat_history = room.chat_history[-MAX_CHAT_HISTORY:]
    touch_room(room)

    await broadcast(room, {
        "type": "chat_message",
        "room_id": room.id,
        "sender_id": system_message.sender_id,
        "sender_name": system_message.sender_name,
        "text": system_message.text,
        "timestamp": system_message.timestamp,
    })


async def handle_chat_message(player_id: str, text: str):
    room_id = player_to_room.get(player_id)
    if not room_id:
        return

    room = rooms.get(room_id)
    if room is None:
        return

    player = room.players.get(player_id)
    if player is None:
        return

    normalized = text.strip()
    if not normalized:
        await send_json(player.websocket, {
            "type": "error",
            "message": "Chat message must not be empty"
        })
        return

    if len(normalized) > MAX_CHAT_MESSAGE_LEN:
        await send_json(player.websocket, {
            "type": "error",
            "message": f"Chat message is too long (max {MAX_CHAT_MESSAGE_LEN} chars)"
        })
        return

    message = ChatMessage(
        sender_id=player.id,
        sender_name=player.name,
        text=normalized,
        timestamp=time.time(),
    )

    room.chat_history.append(message)
    room.chat_history = room.chat_history[-MAX_CHAT_HISTORY:]
    touch_room(room)

    await broadcast(room, {
        "type": "chat_message",
        "room_id": room.id,
        "sender_id": message.sender_id,
        "sender_name": message.sender_name,
        "text": message.text,
        "timestamp": message.timestamp,
    })


async def try_finish_round(room: Room):
    if room.finished:
        return

    if len(room.players) != MAX_PLAYERS_PER_ROOM:
        return

    if len(room.submissions) != MAX_PLAYERS_PER_ROOM:
        return

    result_words = {}
    for player_id, word in room.submissions.items():
        player = room.players.get(player_id)
        if player:
            result_words[player.name] = word

    submitted_words = list(room.submissions.values())
    match = len(set(submitted_words)) == 1
    agreed_word = submitted_words[0] if match else None

    room.history.append({
        "round": room.round_number,
        "words": result_words,
        "match": match,
    })
    touch_room(room)

    await broadcast(room, {
        "type": "round_result",
        "room_id": room.id,
        "round": room.round_number,
        "words": result_words,
        "match": match,
    })

    if match:
        room.finished = True
        touch_room(room)
        await broadcast(room, {
            "type": "game_over",
            "room_id": room.id,
            "round": room.round_number,
            "word": agreed_word,
            "history": room.history,
        })
        await add_system_message(room, f"Игра завершена. Оба игрока сошлись на слове: {agreed_word}")
    else:
        room.submissions.clear()
        room.round_number += 1
        touch_room(room)
        await broadcast_room_state(room)


async def join_room(ws, room_id: str, name: str) -> Optional[str]:
    room = rooms.get(room_id)
    if room is None:
        room = Room(id=room_id)
        rooms[room_id] = room

    if room.finished:
        await send_json(ws, {
            "type": "error",
            "message": "Game already finished in this room"
        })
        return None

    if room.is_full():
        await send_json(ws, {
            "type": "error",
            "message": "Room is full"
        })
        return None

    player_id = str(uuid.uuid4())
    player = Player(
        id=player_id,
        name=name.strip() or f"Player-{player_id[:6]}",
        websocket=ws
    )

    room.players[player_id] = player
    player_to_room[player_id] = room_id
    touch_room(room)

    await send_json(ws, {
        "type": "joined",
        "player_id": player_id,
        "room_id": room_id,
        "name": player.name,
    })

    await send_chat_history(player, room)
    await add_system_message(room, f"{player.name} joined the room")
    await broadcast_room_state(room)

    return player_id


async def submit_word(player_id: str, word: str):
    room_id = player_to_room.get(player_id)
    if not room_id:
        return

    room = rooms.get(room_id)
    if room is None or room.finished:
        return

    if player_id not in room.players:
        return

    normalized = word.strip()
    if not normalized:
        player = room.players[player_id]
        await send_json(player.websocket, {
            "type": "error",
            "message": "Word must not be empty"
        })
        return

    if len(normalized) > MAX_WORD_LEN:
        player = room.players[player_id]
        await send_json(player.websocket, {
            "type": "error",
            "message": f"Word is too long (max {MAX_WORD_LEN} chars)"
        })
        return

    room.submissions[player_id] = normalized
    touch_room(room)
    await broadcast_room_state(room)
    await try_finish_round(room)


async def remove_player(player_id: str):
    room_id = player_to_room.pop(player_id, None)
    if not room_id:
        return

    room = rooms.get(room_id)
    if room is None:
        return

    player = room.players.pop(player_id, None)
    room.submissions.pop(player_id, None)
    touch_room(room)

    if player is not None and room.players:
        await add_system_message(room, f"{player.name} left the room")
        await broadcast_room_state(room)

    if not room.players:
        rooms.pop(room_id, None)


def http_response(status: int, body: bytes, content_type: str) -> bytes:
    reason = HTTP_STATUS_TEXT.get(status, "OK")
    headers = [
        f"HTTP/1.1 {status} {reason}",
        f"Content-Type: {content_type}",
        f"Content-Length: {len(body)}",
        "Cache-Control: no-store",
        "Connection: close",
        "",
        "",
    ]
    return "\r\n".join(headers).encode("ascii") + body


async def send_http(writer, status: int, body: bytes, content_type: str):
    writer.write(http_response(status, body, content_type))
    await writer.drain()
    writer.close()
    await writer.wait_closed()


async def handle_dashboard(reader: asyncio.StreamReader, writer: asyncio.StreamWriter):
    try:
        request_head = await reader.readuntil(b"\r\n\r\n")
    except (asyncio.IncompleteReadError, asyncio.LimitOverrunError):
        await send_http(writer, 400, b"Bad Request", "text/plain; charset=utf-8")
        return

    try:
        request_line = request_head.decode("iso-8859-1").split("\r\n", 1)[0]
        method, target, _ = request_line.split(" ", 2)
    except ValueError:
        await send_http(writer, 400, b"Bad Request", "text/plain; charset=utf-8")
        return

    if method != "GET":
        await send_http(writer, 405, b"Method Not Allowed", "text/plain; charset=utf-8")
        return

    path = urlsplit(target).path

    if path in {"/", "/dashboard"}:
        await send_http(writer, 200, DASHBOARD_HTML.encode("utf-8"), "text/html; charset=utf-8")
        return

    if path == "/api/dashboard/state":
        payload = json.dumps(build_dashboard_state(), ensure_ascii=False).encode("utf-8")
        await send_http(writer, 200, payload, "application/json; charset=utf-8")
        return

    if path == "/healthz":
        await send_http(writer, 200, b"ok", "text/plain; charset=utf-8")
        return

    await send_http(writer, 404, b"Not Found", "text/plain; charset=utf-8")


async def handler(ws):
    current_player_id: Optional[str] = None

    try:
        async for raw_message in ws:
            try:
                data = json.loads(raw_message)
            except json.JSONDecodeError:
                await send_json(ws, {
                    "type": "error",
                    "message": "Invalid JSON"
                })
                continue

            msg_type = data.get("type")

            if msg_type == "join":
                if current_player_id is not None:
                    await send_json(ws, {
                        "type": "error",
                        "message": "Already joined"
                    })
                    continue

                room_id = str(data.get("room_id", "")).strip()
                name = str(data.get("name", "")).strip()

                if not room_id:
                    await send_json(ws, {
                        "type": "error",
                        "message": "room_id is required"
                    })
                    continue

                current_player_id = await join_room(ws, room_id, name)

            elif msg_type == "submit_word":
                if current_player_id is None:
                    await send_json(ws, {
                        "type": "error",
                        "message": "Join a room first"
                    })
                    continue

                word = str(data.get("word", ""))
                await submit_word(current_player_id, word)

            elif msg_type == "chat_message":
                if current_player_id is None:
                    await send_json(ws, {
                        "type": "error",
                        "message": "Join a room first"
                    })
                    continue

                text = str(data.get("text", ""))
                await handle_chat_message(current_player_id, text)

            elif msg_type == "leave":
                if current_player_id is not None:
                    await remove_player(current_player_id)
                    current_player_id = None

                await send_json(ws, {"type": "left"})
                break

            elif msg_type == "ping":
                await send_json(ws, {"type": "pong"})

            else:
                await send_json(ws, {
                    "type": "error",
                    "message": f"Unknown message type: {msg_type}"
                })

    except ConnectionClosed:
        pass
    finally:
        if current_player_id is not None:
            await remove_player(current_player_id)


async def main():
    async with serve(
        handler,
        SERVER_HOST,
        SERVER_PORT,
        ping_interval=20,
        ping_timeout=20,
    ):
        dashboard_server = await asyncio.start_server(
            handle_dashboard,
            DASHBOARD_HOST,
            DASHBOARD_PORT,
        )
        async with dashboard_server:
            print(f"WebSocket server started on ws://{SERVER_HOST}:{SERVER_PORT}")
            print(f"Dashboard UI started on http://{DASHBOARD_HOST}:{DASHBOARD_PORT}")
            await asyncio.Future()


if __name__ == "__main__":
    asyncio.run(main())

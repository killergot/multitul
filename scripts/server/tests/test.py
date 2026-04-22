import asyncio
import json
import argparse
import sys

import websockets


# ----------- utils -----------

async def send(ws, data: dict):
    await ws.send(json.dumps(data, ensure_ascii=False))


def pretty_print(msg: dict):
    t = msg.get("type")

    if t == "chat_message":
        print(f"[CHAT] {msg.get('sender_name')}: {msg.get('text')}")
    elif t == "room_state":
        print(f"[STATE] players={msg.get('players')} "
              f"ready={msg.get('ready_count')}/{msg.get('total_players')} "
              f"round={msg.get('round')} finished={msg.get('finished')}")
    elif t == "round_result":
        print(f"[ROUND {msg.get('round')}] {msg.get('words')} match={msg.get('match')}")
    elif t == "game_over":
        print(f"[GAME OVER] word={msg.get('word')} round={msg.get('round')}")
    elif t == "joined":
        print(f"[JOINED] id={msg.get('player_id')} name={msg.get('name')}")
    elif t == "chat_history":
        print("[CHAT HISTORY]")
        for m in msg.get("messages", []):
            print(f"  {m['sender_name']}: {m['text']}")
    elif t == "error":
        print(f"[ERROR] {msg.get('message')}")
    else:
        print(f"[RAW] {msg}")


# ----------- receive loop -----------

async def receiver(ws):
    try:
        async for message in ws:
            try:
                data = json.loads(message)
            except:
                print("[INVALID JSON]", message)
                continue

            pretty_print(data)

    except websockets.ConnectionClosed:
        print("Connection closed")
        return


# ----------- input loop -----------

async def user_input_loop(ws):
    loop = asyncio.get_event_loop()

    while True:
        try:
            # читаем ввод НЕ блокируя event loop
            text = await loop.run_in_executor(None, sys.stdin.readline)
        except KeyboardInterrupt:
            return

        text = text.strip()

        if not text:
            continue

        # --- команды ---

        if text.startswith("/w "):
            # отправить слово
            word = text[3:].strip()
            await send(ws, {
                "type": "submit_word",
                "word": word
            })

        elif text.startswith("/c "):
            # чат
            msg = text[3:].strip()
            await send(ws, {
                "type": "chat_message",
                "text": msg
            })

        elif text == "/ping":
            await send(ws, {"type": "ping"})

        elif text == "/leave":
            await send(ws, {"type": "leave"})
            return

        elif text == "/help":
            print("""
Команды:
/w слово      -> отправить слово
/c текст      -> отправить сообщение в чат
/ping         -> ping сервер
/leave        -> выйти
/help         -> помощь
""")

        else:
            print("Неизвестная команда. /help")


# ----------- main -----------

async def main():
    parser = argparse.ArgumentParser(description="Test WebSocket client")

    parser.add_argument("--host", default="localhost", help="Server IP (default: localhost)")
    parser.add_argument("--port", default=8765, type=int)
    parser.add_argument("--room", default="test-room")
    parser.add_argument("--name", default="Player")

    args = parser.parse_args()

    uri = f"ws://{args.host}:{args.port}"

    print(f"Connecting to {uri}...")

    async with websockets.connect(uri) as ws:
        print("Connected!")

        # join
        await send(ws, {
            "type": "join",
            "room_id": args.room,
            "name": args.name
        })

        print("Введите /help для списка команд")

        # параллельно:
        # - читаем сервер
        # - читаем ввод
        await asyncio.gather(
            receiver(ws),
            user_input_loop(ws)
        )


if __name__ == "__main__":
    asyncio.run(main())
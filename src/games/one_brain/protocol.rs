#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Join { room_id: String, name: String },
    SubmitWord { word: String },
    ChatMessage { text: String },
    Leave,
    Ping,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ChatItem {
    pub sender_id: String,
    pub sender_name: String,
    pub text: String,
    pub timestamp: f64,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Joined { player_id: String, room_id: String, name: String },
    ChatHistory { room_id: String, messages: Vec<ChatItem> },
    ChatMessage {
        room_id: String,
        sender_id: String,
        sender_name: String,
        text: String,
        timestamp: f64,
    },
    RoomState {
        room_id: String,
        players: Vec<String>,
        ready_count: u32,
        total_players: u32,
        round: u32,
        finished: bool,
    },
    RoundResult {
        room_id: String,
        round: u32,
        words: std::collections::HashMap<String, String>,
        #[serde(rename = "match")]
        is_match: bool,
    },
    GameOver {
        room_id: String,
        round: u32,
        word: String,
        history: Vec<serde_json::Value>,
    },
    Left,
    Pong,
    Error { message: String },
}
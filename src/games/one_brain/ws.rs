use futures::channel::mpsc;
use futures::stream::BoxStream;
use futures::{FutureExt, Sink, SinkExt, StreamExt};
use iced::{Subscription, stream};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use crate::games::one_brain::menu::BrainMessage;
use crate::games::one_brain::protocol::{ClientMessage, ServerMessage};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct WsConfig {
    pub server_url: String,
    pub room_id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum WsCommand {
    Send(ClientMessage),
}

pub fn subscription(config: WsConfig) -> Subscription<BrainMessage> {
    Subscription::run_with(config, connect_worker)
}

fn connect_worker(config: &WsConfig) -> BoxStream<'static, BrainMessage> {
    let config = config.clone();

    Box::pin(stream::channel(100, async move |mut output| {
        let (sender, mut receiver) = mpsc::channel(100);
        let _ = output.send(BrainMessage::WsReady(sender)).await;

        let connection = connect_async(config.server_url.as_str()).await;
        let (ws_stream, _) = match connection {
            Ok(connection) => connection,
            Err(error) => {
                let _ = output
                    .send(BrainMessage::WsError(format!(
                        "Не удалось подключиться: {}",
                        error
                    )))
                    .await;
                let _ = output.send(BrainMessage::WsClosed).await;
                return;
            }
        };

        let _ = output.send(BrainMessage::WsConnected).await;

        let (mut writer, mut reader) = ws_stream.split();

        let join_message = ClientMessage::Join {
            room_id: config.room_id,
            name: config.name,
        };

        if let Err(error) = send_json(&mut writer, &join_message).await {
            let _ = output
                .send(BrainMessage::WsError(format!(
                    "Не удалось отправить join: {}",
                    error
                )))
                .await;
            let _ = output.send(BrainMessage::WsClosed).await;
            return;
        }

        loop {
            let next_command = receiver.next().fuse();
            let next_frame = reader.next().fuse();
            futures::pin_mut!(next_command, next_frame);

            futures::select! {
                maybe_command = next_command => {
                    match maybe_command {
                        Some(WsCommand::Send(message)) => {
                            if let Err(error) = send_json(&mut writer, &message).await {
                                let _ = output
                                    .send(BrainMessage::WsError(format!(
                                        "Не удалось отправить сообщение: {}",
                                        error
                                    )))
                                    .await;
                                let _ = output.send(BrainMessage::WsClosed).await;
                                break;
                            }
                        }
                        None => {
                            let _ = output.send(BrainMessage::WsClosed).await;
                            break;
                        }
                    }
                }
                maybe_frame = next_frame => {
                    match maybe_frame {
                        Some(Ok(Message::Text(payload))) => {
                            match serde_json::from_str::<ServerMessage>(payload.as_ref()) {
                                Ok(message) => {
                                    let _ = output.send(BrainMessage::WsEvent(message)).await;
                                }
                                Err(error) => {
                                    let _ = output
                                        .send(BrainMessage::WsError(format!(
                                            "Не удалось разобрать ответ сервера: {}",
                                            error
                                        )))
                                        .await;
                                }
                            }
                        }
                        Some(Ok(Message::Close(_))) => {
                            let _ = output.send(BrainMessage::WsClosed).await;
                            break;
                        }
                        Some(Ok(_)) => {}
                        Some(Err(error)) => {
                            let _ = output
                                .send(BrainMessage::WsError(format!(
                                    "Ошибка WebSocket: {}",
                                    error
                                )))
                                .await;
                            let _ = output.send(BrainMessage::WsClosed).await;
                            break;
                        }
                        None => {
                            let _ = output.send(BrainMessage::WsClosed).await;
                            break;
                        }
                    }
                }
            }
        }
    }))
}

async fn send_json<S>(writer: &mut S, message: &ClientMessage) -> Result<(), String>
where
    S: Sink<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
{
    let payload = serde_json::to_string(message).map_err(|error| error.to_string())?;
    writer
        .send(Message::Text(payload.into()))
        .await
        .map_err(|error| error.to_string())
}

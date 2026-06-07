use std::sync::Arc;

use meshcore_rs::{
    EventPayload, EventType,
    commands::{CommandHandler, Destination},
};

use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

use crate::meshcore_proto::{
    ChannelMessage, ContactMessage, ReceiveMessageResponse, SendMessageRequest, SendMessageResponse, receive_message_response::Payload, send_message_request::Destination as ProtoDestination
};

pub async fn receive_message(
    commands: &Arc<Mutex<CommandHandler>>,
) -> Result<Response<ReceiveMessageResponse>, Status> {
    let event_opt = {
        let cmd = commands.lock().await;
        cmd.get_msg().await.map_err(|e| {
            error!(error = %e, "get_msg failed");
            Status::internal("Failed to get message")
        })?
    };

    debug!("get_msg returned event: {:?}", event_opt);

    let Some(event) = event_opt else {
        return Ok(Response::new(ReceiveMessageResponse {
            payload: None,
        }));
    };

    let resp = match event.event_type {
        EventType::ContactMsgRecv => {
            if let EventPayload::ContactMessage(msg) = event.payload {
                info!(sender = %hex::encode(msg.sender_prefix), "Received contact messasge");
                ReceiveMessageResponse {
                    payload: Some(Payload::ContactMessage(ContactMessage {
                        sender_prefix_hex: hex::encode(msg.sender_prefix),
                        text: msg.text,
                    }))
                }
            } else {
                error!("ContactMsgRecv event missing payload");
                return Err(Status::internal("ContactMsgRecv event missing payload"));
            }
        }
        EventType::ChannelMsgRecv => {
            if let EventPayload::ChannelMessage(msg) = event.payload {
                info!(channel = msg.channel_idx, "Received channel messasge");
                ReceiveMessageResponse {
                    payload: Some(Payload::ChannelMessage(ChannelMessage {
                        channel_index: msg.channel_idx as u32,
                        text: msg.text,
                    }))
                }
            } else {
                error!("ChannelMsgRecv event missing payload");
                return Err(Status::internal("ChannelMsgRecv event missing payload"));
            }
        }
        other => {
            error!("Received non-message event: {:?}", other);
            return Err(Status::internal(format!(
                "unexpected event type from get_msg: {other:?}"
            )));
        }
    };

    Ok(Response::new(resp))
}

pub async fn send_message(
    command: &Arc<Mutex<CommandHandler>>,
    request: Request<SendMessageRequest>,
) -> Result<Response<SendMessageResponse>, Status> {
    let req = request.into_inner();
    let text = &req.text;

    // Convert thei 
    let timestamp = if let Some(d) = req.sent_at {
        Some(d.seconds as u32)
    } else {
        None
    };
    
    let result = {
        let cmd = command.lock().await;
        match req.destination {
            Some(ProtoDestination::ContactPubkeyHex(ref hex)) => {
                cmd
                    .send_msg(Destination::Hex(hex.to_string()), text, timestamp)
                    .await
                    .map(|_| ())
            }
            Some(ProtoDestination::ChannelIndex(idx)) => {
                cmd.send_channel_msg(idx as u8, text, timestamp).await
            }
            None => {
                return Err(Status::invalid_argument(
                    "destination must be set (contact_pubkey_hex or channel_index)",
                ));
            }
        }
    };

    if let Err(ref e) = result {
        error!(error = %e, "Send message failed");
        Err(Status::internal("Failed to send message "))
    } else {
        Ok(Response::new(SendMessageResponse {}))
    }
}

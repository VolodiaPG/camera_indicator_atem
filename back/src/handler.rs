use crate::{ws, Client, Clients, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, ws::Message, Reply};

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    camera_id: usize,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    topic: String,
    user_id: Option<usize>,
    message: String,
}

#[derive(Debug)]
struct CameraIdOutRangeError;
impl warp::reject::Reject for CameraIdOutRangeError {}

pub async fn publish_handler(body: Event, clients: Clients) -> Result<impl Reply> {
    clients
        .read()
        .await
        .iter()
        .filter(|(_, client)| match body.user_id {
            Some(v) => client.camera_id == v,
            None => true,
        })
        .filter(|(_, client)| client.topics.contains(&body.topic))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(body.message.clone())));
            }
        });

    Ok(StatusCode::OK)
}

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
    let camera_id = body.camera_id;
    let uuid = Uuid::new_v4().simple().to_string();

    match register_client(uuid.clone(), camera_id, clients).await {
        Ok(()) => {
            return Ok(json(&RegisterResponse {
                url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
            }))
        }
        Err(err) => return Err(err),
    }
}

async fn register_client(id: String, camera_id: usize, clients: Clients) -> Result<()> {
    if camera_id >= 8 {
        return Err(warp::reject::custom(CameraIdOutRangeError));
    }

    clients.write().await.insert(
        id,
        Client {
            camera_id,
            topics: vec![String::from("atem")],
            sender: None,
        },
    );

    Ok(())
}

pub async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
    clients.write().await.remove(&id);
    Ok(StatusCode::OK)
}

pub async fn ws_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
    let client = clients.read().await.get(&id).cloned();
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, id, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}

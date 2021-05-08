#[macro_use]
extern crate log;
use pretty_env_logger;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection};

mod handler;
mod ws;

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String, Client>>>;
type AtemCameraStatus = Arc<RwLock<AtemCameraStatusData>>;

#[derive(Debug, Clone)]
pub struct Client {
    pub camera_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AtemCameraStatusData {
    pub preview: usize,
    pub air: usize,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));
    let camera_status: AtemCameraStatus =
        Arc::new(RwLock::new(AtemCameraStatusData { preview: 0, air: 0 }));

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let register = warp::path("register");
    let register_routes = register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::register_handler)
        .or(register
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_clients(clients.clone()))
            .and_then(handler::unregister_handler));

    let publish = warp::path!("publish")
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::publish_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(handler::ws_handler);

    let routes = health_route
        .or(register_routes)
        .or(ws_route)
        .or(publish)
        .with(warp::cors().allow_any_origin());

    info!("Running server");
    tokio::spawn(get_atem_status(camera_status.clone(), clients.clone()));

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    info!("Server has stopped.");
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

async fn get_atem_status(camera_status: AtemCameraStatus, clients: Clients) {
    loop {
        let mut current = camera_status.read().await.clone();
        current.air += 1;
        current.preview += 1;

        {
            let mut val = camera_status.write().await;
            *val = current.clone();
        }

        let response = serde_json::to_string(&current).unwrap();

        clients
            .read()
            .await
            .iter()
            // .filter(|(_, client)| client.topics.contains(&body.topic))
            .for_each(move |(_, client)| {
                if let Some(sender) = &client.sender {
                    let _ = sender.send(Ok(Message::text(response.clone())));
                }
            });

        tokio::time::delay_for(std::time::Duration::from_millis(100)).await;
    }
}

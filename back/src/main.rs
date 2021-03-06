#[macro_use]
extern crate log;
use pretty_env_logger;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection};

mod handler;
mod ws;

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String, Client>>>;
type AtemCameraStatus = Arc<RwLock<AtemCameraStatusData>>;

// Web Socket client structure
#[derive(Debug, Clone)]
pub struct Client {
    pub camera_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
    pub date_creation: std::time::SystemTime,
}

// Struct to describe the status of the atem
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AtemCameraStatusData {
    pub preview: usize,
    pub air: usize,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "websocket-server", rename_all = "kebab-case")]
/// Start a server that will update regularly the status of the atem's camera.
/// Connect the related client to it to see the display in pseudo realtime
struct Opts {
    /// Ip address of this server. This ip is sent to the receivers for them to connect back to the server using websockets.
    /// Therefore it is recommended to use the ip you choose to expose to your clients (cameramen)
    #[structopt(short, long)]
    ip: String,
    /// Port of this server
    #[structopt(short, long, default_value = "8000")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let opts = Opts::from_args();
    pretty_env_logger::init();

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));
    let camera_status: AtemCameraStatus =
        Arc::new(RwLock::new(AtemCameraStatusData { preview: 0, air: 0 }));

    let route_prefix = warp::path!("api" / ..);

    let websocket_url = format!("ws://{}:{}/api/ws/", opts.ip, opts.port);

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let register = warp::path("register");
    let register_routes = register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and(with_url(websocket_url.clone()))
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

    let publish_atem = warp::path!("atem")
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and(with_atem_status(camera_status.clone()))
        .and_then(handler::publish_atem_status_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(handler::ws_handler);

    let api_routes = route_prefix.and(
        health_route
            .or(register_routes)
            .or(ws_route)
            .or(publish)
            .or(publish_atem)
            .recover(recover_api) // all other mappings that start with /api will be 404s
            .with(warp::cors().allow_any_origin()),
    );

    // route to serve the static files of the front
    // the `or` makes sure to redirect all other request directly to the SPA application
    // for example you don't want the SPA router to be interpreted as a request to the backend
    let front_route = warp::fs::dir("www").or(warp::fs::file(format!("{}/index.html", "www")));

    let routes = api_routes.or(front_route);

    info!("Running server");

    warp::serve(routes).run(([0, 0, 0, 0], opts.port)).await;
    info!("Server has stopped.");
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_atem_status(
    status: AtemCameraStatus,
) -> impl Filter<Extract = (AtemCameraStatus,), Error = Infallible> + Clone {
    warp::any().map(move || status.clone())
}

fn with_url(url: String) -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::any().map(move || url.clone())
}

async fn recover_api(_: Rejection) -> Result<impl warp::Reply> {
    Ok(warp::http::StatusCode::NOT_FOUND)
}

async fn send_status(
    camera_status: AtemCameraStatus,
    clients: Clients,
    new_status: AtemCameraStatusData,
) {
    let current = camera_status.read().await.clone();

    if current != new_status {
        let mut val = camera_status.write().await;
        *val = new_status.clone();
    }

    let response = serde_json::to_string(&new_status).unwrap();

    clients.read().await.iter().for_each(move |(_, client)| {
        // if duration_since sends an error, that means that value as an argument is earlier thant the one comparing
        if current != new_status {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(response.clone())));
            }
        }
    });
}

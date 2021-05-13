#[macro_use]
extern crate log;
use pretty_env_logger;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{collections::HashMap, process::Command};
use std::{convert::Infallible};
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
    /// Path of the script being called to get the atem status
    #[structopt(short, long)]
    atem_script: String,
    /// Ip address of this server
    #[structopt(short, long, default_value = "127.0.0.1")]
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

    let routes = route_prefix.and(
        health_route
            .or(register_routes)
            .or(ws_route)
            .or(publish)
            .or(publish_atem)
            .with(warp::cors().allow_any_origin()),
    );

    info!("Running server");
    tokio::spawn(get_atem_status(
        camera_status.clone(),
        clients.clone(),
        opts.atem_script.clone(),
    ));

    warp::serve(routes).run(([0, 0, 0, 0], opts.port)).await;
    info!("Server has stopped.");
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_atem_status(status: AtemCameraStatus) -> impl Filter<Extract = (AtemCameraStatus,), Error = Infallible> + Clone {
    warp::any().map(move || status.clone())
}

fn with_url(url: String) -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::any().map(move || url.clone())
}

async fn get_atem_status(camera_status: AtemCameraStatus, clients: Clients, script: String) {
    // let mut last_time_refreshed = std::time::SystemTime::now();

    loop {
        let script_response = get_atem_results(script.clone());

        send_status(camera_status.clone(), clients.clone(), script_response).await;

        // last_time_refreshed = std::time::SystemTime::now();

        tokio::time::delay_for(std::time::Duration::from_millis(100)).await;
    }
}

async fn send_status(camera_status: AtemCameraStatus,  clients: Clients, new_status: AtemCameraStatusData){
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

fn get_atem_results(script: String) -> AtemCameraStatusData {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(script.clone()).output()
    } else {
        Command::new("sh").arg("-c").arg(script.clone()).output()
    }
    .expect(
        &format!(
            "failed to execute process from script {}",
            &script.clone()[..]
        )[..],
    );

    if !output.status.success() {
        warn!(
            "Calling the script {} resulted in status {}",
            script, output.status
        );
    }

    if output.stderr.len() > 0 {
        warn!(
            "Calling the script {} resulted in stderr: {}",
            script,
            String::from_utf8(output.stderr.clone()).unwrap()
        );
    }

    serde_json::from_str(&String::from_utf8(output.stdout).unwrap()[..]).unwrap()
}

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket;

mod models;

use models::AtemCamStatusJson;
use rocket_contrib::json::Json;

#[get("/status")]
fn index() -> Json<AtemCamStatusJson>{
    Json(AtemCamStatusJson {
        cam_prod: 1,
        cam_preview: 2,
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}

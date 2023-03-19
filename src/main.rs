mod rest_controllers;
mod spotify;
use crate::rest_controllers::{login, play, pause, resume, set_volume};

#[macro_use] extern crate rocket;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![login, play, pause, resume, set_volume])
}

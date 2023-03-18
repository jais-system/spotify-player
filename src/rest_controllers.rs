use librespot::discovery::Credentials;
use librespot::protocol::authentication::AuthenticationType;
use rocket::http::{Status};
use rocket::serde::{Deserialize, json::Json};
use crate::spotify::{SpotifyPlayer};
use once_cell::sync::OnceCell;

static mut INSTANCE: OnceCell<SpotifyPlayer> = OnceCell::new();

#[derive(Deserialize)]
pub struct Login<'a> {
    #[serde(rename = "accessToken")]
    pub access_token: &'a str,
}

#[post("/login", format = "application/json", data = "<login>")]
pub async fn login(login: Json<Login<'_>>) -> Status {
    let credentials = Credentials {
        username: "".to_string(),
        auth_type: AuthenticationType::AUTHENTICATION_SPOTIFY_TOKEN,
        auth_data: login.access_token.as_bytes().to_vec(),
    };

    let mut service = SpotifyPlayer::default();

    if let Err(error) = service.login(credentials).await {
        println!("{}", error);
        return Status::InternalServerError;
    }

    unsafe {
        if let Err(_) = INSTANCE.set(service) {
            return Status::InternalServerError;
        }
    }

    return Status::Ok;
}

#[post("/play/<track_uri>")]
pub async fn play(track_uri: &str) -> Status {
    unsafe {
        if let Some(spotify) = INSTANCE.get_mut() {
            if let Err(error) = spotify.play(track_uri).await {
                println!("{}", error);

                return Status::InternalServerError;
            }
        }
    }

    return Status::Ok;
}

#[post("/pause")]
pub fn pause() -> Status {
    unsafe {
        if let Some(spotify) = INSTANCE.get_mut() {
            if let Err(error) = spotify.pause() {
                println!("{}", error);

                return Status::InternalServerError;
            }
        }
    }

    return Status::Ok;
}

#[post("/resume")]
pub fn resume() -> Status {
    unsafe {
        if let Some(spotify) = INSTANCE.get_mut() {
            if let Err(error) = spotify.resume() {
                println!("{}", error);

                return Status::InternalServerError;
            }
        }
    }

    return Status::Ok;
}

#[post("/volume/<volume>")]
pub fn set_volume(volume: u16) -> Status {
    unsafe {
        if let Some(spotify) = INSTANCE.get_mut() {
            spotify.set_volume(volume)
        }
    }

    return Status::Ok;
}

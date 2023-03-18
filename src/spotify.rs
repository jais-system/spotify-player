use librespot::core::config::SessionConfig;
use librespot::core::session::{Session};
use librespot::core::spotify_id::{SpotifyId};
use librespot::discovery::Credentials;
use librespot::playback::{audio_backend, mixer};
use librespot::playback::config::{AudioFormat, PlayerConfig};
use librespot::playback::mixer::{Mixer, MixerConfig};
use librespot::playback::player::{Player};
use thiserror::Error;
use anyhow::Result;

#[derive(Default)]
pub struct SpotifyPlayer {
    player: Option<Player>,
    mixer: Option<Box<dyn Mixer>>
}

#[derive(Error, Debug)]
pub enum SpotifyPlayerErrors {
    #[error("Failed to find audio backend")]
    FailedToFindAudioBackend,

    #[error("Session connect failed")]
    SessionConnectFailed,

    #[error("Invalid track URI")]
    InvalidTrackUri,

    #[error("Mixer not found")]
    NoMixerFound
}

impl SpotifyPlayer {
    pub async fn login(&mut self, credentials: Credentials) -> Result<(), SpotifyPlayerErrors> {
        let audio_format = AudioFormat::default();
        let session_config = SessionConfig::default();
        let player_config = PlayerConfig::default();

        let backend = match audio_backend::find(None) {
            None => return Err(SpotifyPlayerErrors::FailedToFindAudioBackend),
            Some(value) => value
        };

        let session = match Session::connect(session_config, credentials, None, false).await {
            Ok(value) => value.0,
            Err(_) => return Err(SpotifyPlayerErrors::SessionConnectFailed)
        };

        let soft_mixer = match mixer::find(None) {
            Some(value) => value(MixerConfig::default()),
            None => return Err(SpotifyPlayerErrors::NoMixerFound)
        };

        let player = Player::new(player_config, session, soft_mixer.get_soft_volume(), move || {
            backend(None, audio_format)
        });

        self.mixer = Some(soft_mixer);
        self.player = Some(player.0);

        return Ok(());
    }

    pub async fn play(&mut self, track_uri: &str) -> Result<(), SpotifyPlayerErrors> {
        let track = match SpotifyId::from_uri(track_uri) {
            Ok(value) => value,
            Err(_) => return Err(SpotifyPlayerErrors::InvalidTrackUri)
        };

        if let Some(ref mut player) = self.player {
            player.load(track, true, 0);
        }

        return Ok(());
    }

    pub fn pause(&mut self) -> Result<(), SpotifyPlayerErrors> {
        if let Some(ref mut player) = self.player {
            player.pause();
        }

        return Ok(());
    }

    pub fn resume(&mut self) -> Result<(), SpotifyPlayerErrors> {
        if let Some(ref mut player) = self.player {
            player.play();
        }

        return Ok(());
    }

    pub fn set_volume(&mut self, volume: u16) {
        if let Some(mixer) = &self.mixer {
            mixer.set_volume(volume);
        }
    }
}

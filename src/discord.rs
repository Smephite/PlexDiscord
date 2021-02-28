use super::session_status::PlaybackStatus;
use rustcord::{EventHandlers, RichPresenceBuilder, Rustcord, User};
use std::ops::Add;
use std::time::{Duration, SystemTime};

pub struct Handlers;

impl EventHandlers for Handlers {
    fn ready(user: User) {
        println!("User {}#{} logged in...", user.username, user.discriminator);
    }
}

pub fn update(
    discord: &Rustcord,
    status: &PlaybackStatus,
) -> Result<(), std::ffi::NulError> {
    discord.run_callbacks();

    if status.state != "playing" {
        return discord.update_presence(
            RichPresenceBuilder::new()
                .state(&super::format::state(status))
                .details(&super::format::details(status))
                .build(),
        );
    }
    else {
        return discord.update_presence(
            RichPresenceBuilder::new()
                .state(&super::format::state(status))
                .details(&super::format::details(status))
                .end_time(SystemTime::now().add(Duration::from_millis(
                    (status.duration - status.current) as u64,
                )))
                .build(),
        );
    }
}

pub fn clear(discord: &Rustcord) {
    discord.run_callbacks();
    discord.clear_presence();
}

pub fn initialize(app_id: String) -> Result<Rustcord, std::ffi::NulError> {
    Rustcord::init::<Handlers>(&app_id, true, None)
}

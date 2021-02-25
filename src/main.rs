extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate json_config;

mod config;
mod discord;
mod format;
mod session_status;

fn main() {
    let config = config::fetch_config();

    if config.is_none() {
        eprintln!("Error while reading config file!");
        std::process::exit(1);
    }
    let config: config::Configuration = config.unwrap();

    let dc = discord::initialize(config.discord_app_id).unwrap();

    let mut was_ok = true;
    println!("Starting Plex <-> Discord Rich Presence!");
    loop {
        let data = session_status::fetch_session_status(
            &config.plex_token,
            &config.plex_user,
            &config.plex_server,
        );
        
        if data.is_ok() {

            let data = data.unwrap();

            
            if !was_ok {
                println!("Updating Discord Rich Presence => {:?}", data);
            }

            let res = discord::update(&dc, &data);
            if res.is_err() {
                println!("Failed to push to Discord! {:?}", res);
            }
            was_ok = true;
        } else {
            if was_ok {
                println!("Nothing playing => Resetting Rich Presence!");
            }
            discord::clear(&dc);
            was_ok = false;
        }
    }
}

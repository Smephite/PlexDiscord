use json_config::ConfigurationBuilder;
use json_config::ConfigurationDefinitionParams;
use json_config::ConfigurationSource;
use serde_derive::Deserialize;
use std::io::Read;
use std::io::Write;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub plex_token: String,
    pub plex_user: String,
    pub plex_server: String,
    pub discord_app_id: String,
}

pub fn fetch_config() -> Option<Configuration> {
    let f = std::fs::File::open("config.json");
    let mut file: std::fs::File;

    let mut file_content = String::from("{}");

    let ok = f.is_ok();

    if ok {
        file = f.unwrap();
        file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();
    }
    let builder = json_config::config!(vec![
        json_config::from_json!({
            "plex_token": "",
            "plex_user": "",
            "plex_server": "",
            "discord_app_id": ""
        }),
        json_config::from_str!(file_content),
    ]);

    if !ok {
        let f = std::fs::File::create("config.json");
        file = f.unwrap();
        if file
            .write_all(builder.to_string_pretty().as_bytes())
            .is_err()
        {
            return None;
        }
    }

    let c: Configuration = serde_json::from_str(builder.to_string().as_str()).unwrap();

    Some(c)
}

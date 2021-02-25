use restson::*;
use serde_aux::prelude::*;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    #[serde(rename = "title")]
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Player {
    state: String,
}

fn empty() -> String {
    "".to_string()
}
fn zero() -> usize {
    0usize
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    #[serde(default = "empty")]
    grandparent_title: String,
    #[serde(rename = "librarySectionTitle")]
    library: String,
    title: String,
    #[serde(rename = "User")]
    user: User,
    #[serde(rename = "Player")]
    player: Player,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    duration: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    view_offset: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    year: usize,
    #[serde(rename = "type")]
    library_type: String,
    #[serde(default = "zero", deserialize_with = "deserialize_number_from_string")]
    index: usize,
    #[serde(default = "zero", deserialize_with = "deserialize_number_from_string")]
    parent_index: usize,
}

#[derive(serde_derive::Deserialize, Debug)]
pub struct MediaContainer {
    #[serde(rename = "Metadata")]
    metadata: Vec<MetaData>,
}

#[derive(serde_derive::Deserialize, Debug)]
pub struct SessionStatusBin {
    #[serde(rename = "MediaContainer")]
    media_container: MediaContainer,
}

impl RestPath<()> for SessionStatusBin {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("status/sessions"))
    }
}

#[derive(Debug)]
pub struct PlaybackStatus {
    pub state: String,
    pub series_title: String,
    pub title: String,
    pub user: String,
    pub duration: usize,
    pub current: usize,
    pub library: String,
    pub year: usize,
    pub library_type: String,
    pub season: usize,
    pub episode: usize,
}

#[warn(unused_must_use)]
pub fn fetch_session_status(
    token: &str,
    name: &str,
    server: &str,
) -> Result<PlaybackStatus, Error> {
    let mut client = RestClient::new(server).unwrap();

    client.set_header("X-Plex-Token", token)?;
    client.set_header("Accept", "application/json")?;

    let data = client.get(());

    if data.is_err() {
        // empty
        return Err(restson::Error::RequestError);
    }

    let data: SessionStatusBin = data.unwrap();

    let session = data
        .media_container
        .metadata
        .into_iter()
        .filter(|z: &MetaData| z.user.name.eq(name))
        .next();

    if session.is_none() {
        return Err(restson::Error::InvalidValue);
    }

    let session = session.unwrap();
    Ok(PlaybackStatus {
        state: session.player.state,
        series_title: session.grandparent_title,
        title: session.title,
        user: session.user.name,
        duration: session.duration,
        current: session.view_offset,
        library: session.library,
        year: session.year,
        episode: session.index,
        season: session.parent_index,
        library_type: session.library_type,
    })
}

use super::session_status::PlaybackStatus;

pub fn details(status: &PlaybackStatus) -> String {
    match status.library.to_lowercase().as_str() {
        "lectures" => format!("{}, {}", status.series_title, status.title),
        _ => unknown_library_details(status),
    }
}

fn unknown_library_details(status: &PlaybackStatus) -> String {
    match status.library_type.as_str() {
        "episode" => format!(
            "{} - {} (S{:02}E{:02})",
            status.series_title, status.title, status.season, status.episode
        ),
        _ => format!(
            "{}{} ({})",
            status.series_title.clone()
                + match status.series_title.as_str() {
                    "" => "",
                    _ => " ",
                },
            status.title,
            status.year
        ),
    }
}

pub fn state(status: &PlaybackStatus) -> String {
    let mut state = String::from(match status.state.as_str() {
        "playing" => "▶️ playing",
        "paused" => "⏸️ paused",
        "buffering" => "🍺 buffering",
        _ => "🍺",
    });

    state = state + " ";
    state
        + match status.library.to_lowercase().as_str() {
            "movies" => "Movie",
            "tv shows" => "Series",
            "lectures" => "Lecture",
            _ => status.library.as_str(),
        }
}

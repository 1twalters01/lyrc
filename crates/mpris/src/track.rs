use std::collections::HashMap;

use chrono::Duration;
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Track {
    pub album: Option<String>,
    pub disc_number: Option<i32>,
    pub title: String,
    pub track_number: Option<i32>,
    pub duration: Duration,
    pub id: String,
    pub genres: Vec<String>,
    pub artists: Vec<String>,
    pub album_artists: Vec<String>,
}

impl Track {
    pub fn parse_track(metadata: HashMap<String, OwnedValue>) -> Track {
        println!("metadata: {:#?}", &metadata);
        let album = get_optional_string(&metadata, "xesam:album");
        let disc_number = get_optional_i32(&metadata, "xesam:discNumber");
        let title = get_string(&metadata, "xesam:title");
        let track_number = get_optional_i32(&metadata, "xesam:trackNumber");
        let duration = get_duration(&metadata, "mpris:length");
        let id = get_string(&metadata, "mpris:trackid");
        let genres = get_string_array(&metadata, "xesam:genre");
        let artists = get_string_array(&metadata, "xesam:artist");
        let album_artists = get_string_array(&metadata, "xesam:albumArtist");

        Track {
            album,
            disc_number,
            title,
            track_number,
            duration,
            id,
            genres,
            artists,
            album_artists,
        }
    }
}

fn get_string(metadata: &HashMap<String, OwnedValue>, key: &str) -> String {
    metadata
        .get(key)
        .and_then(|v| v.downcast_ref::<String>().ok())
        .unwrap_or_default()
}

fn get_optional_string(metadata: &HashMap<String, OwnedValue>, key: &str) -> Option<String> {
    metadata
        .get(key)
        .and_then(|v| v.downcast_ref::<String>().ok())
}

fn get_optional_i32(metadata: &HashMap<String, OwnedValue>, key: &str) -> Option<i32> {
    metadata
        .get(key)
        .and_then(|v| v.downcast_ref::<i32>().ok())
}

fn get_string_array(metadata: &HashMap<String, OwnedValue>, key: &str) -> Vec<String> {
    let Some(value) = metadata.get(key) else {
        return Vec::new();
    };

    let value: &Value = value;

    match value {
        Value::Array(array) => array
            .iter()
            .filter_map(|v| String::try_from(v).ok())
            .collect(),
        _ => vec![],
    }
}

fn get_duration(metadata: &HashMap<String, OwnedValue>, key: &str) -> Duration {
    metadata
        .get(key)
        .and_then(|v| v.downcast_ref::<i64>().ok())
        .map(|v| Duration::microseconds(v))
        .unwrap_or_default()
}


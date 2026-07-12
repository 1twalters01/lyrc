use std::{collections::HashMap, path::PathBuf};

use chrono::Duration;
use url::Url;
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug, Clone)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artists: Vec<String>,
    pub album: Option<String>,
    pub url: Option<PathBuf>,
    pub duration: Duration,
}

impl Track {
    pub fn parse_track(metadata: HashMap<String, OwnedValue>) -> Track {
        let id = get_string(&metadata, "mpris:trackid");
        let title = get_string(&metadata, "xesam:title");
        let artists = get_array(&metadata, "xesam:artist");
        let album = get_optional_string(&metadata, "xesam:album");
        let url = get_url(&metadata, "xesam:url");
        let duration = get_duration(&metadata);

        Track {
            id,
            title,
            artists,
            album,
            url,
            duration,
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

fn get_array(metadata: &HashMap<String, OwnedValue>, key: &str) -> Vec<String> {
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

fn get_url(metadata: &HashMap<String, OwnedValue>, key: &str) -> Option<PathBuf> {
    metadata
        .get(key)
        .and_then(|v| v.downcast_ref::<String>().ok())
        .and_then(|s| Url::parse(&s).ok())
        .and_then(|url| {
            if url.scheme() == "file" {
                url.to_file_path().ok()
            } else {
                None
            }
        })
}

fn get_duration(metadata: &HashMap<String, OwnedValue>) -> Duration {
    metadata
        .get("mpris:length")
        .and_then(|v| v.downcast_ref::<i64>().ok())
        .map(|v| Duration::microseconds(v))
        .unwrap_or_default()
}

use std::{collections::HashMap, path::PathBuf};

use async_stream::stream;
use chrono::Duration;
use futures_core::Stream;
use futures_util::StreamExt;
use url::Url;
use zbus::{
    Connection,
    fdo::{PropertiesChanged, PropertiesProxy},
    names::OwnedWellKnownName,
    zvariant::{OwnedValue, Value},
};

use crate::{
    proxy::PlayerProxy,
    track::{MprisEvents, PlaybackCommand, PlaybackStatus, PlayerEvent, Track},
};

pub struct MprisClient {
    connection: Connection,
    service: OwnedWellKnownName,
}

impl MprisClient {
    pub async fn connect(player: &str) -> zbus::Result<Self> {
        let connection = Connection::session().await?;
        let service = format!("org.mpris.MediaPlayer2.{player}");

        Ok(Self {
            connection,
            service: service.try_into()?,
        })
    }

    async fn proxy(&self) -> zbus::Result<PlayerProxy<'_>> {
        PlayerProxy::builder(&self.connection)
            .destination(&self.service)?
            .build()
            .await
    }

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

    pub async fn position(&self) -> zbus::Result<Duration> {
        let proxy = self.proxy().await?;
        let micros = proxy.position().await?;
        Ok(Duration::microseconds(micros as i64))
    }

    pub async fn metadata(&self) -> zbus::Result<HashMap<String, OwnedValue>> {
        let proxy = self.proxy().await?;
        Ok(proxy.metadata().await?)
    }

    pub async fn current_track(&self) -> zbus::Result<Track> {
        let metadata = self.proxy().await?.metadata().await?;

        Ok(Self::parse_track(metadata))
    }

    pub async fn playback_status(&self) -> zbus::Result<PlaybackStatus> {
        let proxy = self.proxy().await?;
        let status = proxy.playback_status().await?;

        Ok(match status.as_str() {
            "Playing" => PlaybackStatus::Playing,
            "Paused" => PlaybackStatus::Paused,
            "Stopped" => PlaybackStatus::Stopped,
            _ => PlaybackStatus::Unknown,
        })
    }

    pub async fn execute(&self, command: PlaybackCommand) -> zbus::Result<()> {
        let proxy = self.proxy().await?;

        match command {
            PlaybackCommand::Play => proxy.play().await?,
            PlaybackCommand::Pause => proxy.pause().await?,
            PlaybackCommand::Toggle => proxy.play_pause().await?,
            PlaybackCommand::Next => proxy.next().await?,
            PlaybackCommand::Previous => proxy.previous().await?,
            PlaybackCommand::Seek(offset) => {
                proxy.seek(offset.num_microseconds().unwrap_or(0)).await?
            }
        }

        Ok(())
    }

    pub async fn events(&self) -> zbus::Result<impl Stream<Item = PlayerEvent>> {
        let proxy = PropertiesProxy::builder(&self.connection)
            .destination(&self.service)?
            .build()
            .await?;

        let client = self.clone();
        let mut signals = proxy.receive_properties_changed().await?;

        let output = stream! {
            while let Some(signal) = signals.next().await {
                let Ok(args) = signal.args() else {
                    continue;
                };

                let changed = args.changed_properties();

                if changed.contains_key("Metadata") {
                    if let Ok(track) = client.current_track().await {
                        yield PlayerEvent::TrackChanged(track);
                    }
                }

                if changed.contains_key("PlaybackStatus") {
                    if let Ok(status) = client.playback_status().await {
                        yield PlayerEvent::PlaybackChanged(status);
                    }
                }
            }
        };

        Ok(Box::pin(output))
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

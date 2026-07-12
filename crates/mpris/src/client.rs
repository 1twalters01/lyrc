use std::collections::HashMap;

use async_stream::stream;
use chrono::Duration;
use futures_core::Stream;
use futures_util::StreamExt;
use zbus::{Connection, fdo::PropertiesProxy, names::OwnedWellKnownName, zvariant::OwnedValue};

use crate::{
    playback::{PlaybackCommand, PlaybackStatus, PlayerEvent},
    proxy::PlayerProxy,
    track::Track,
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

        Ok(Track::parse_track(metadata))
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

        let mut signals = proxy.receive_properties_changed().await?;

        let output = stream! {
            while let Some(signal) = signals.next().await {
                let Ok(args) = signal.args() else {
                    continue;
                };

                let changed = args.changed_properties();

                if changed.contains_key("Metadata") {
                    if let Ok(track) = self.current_track().await {
                        yield PlayerEvent::TrackChanged(track);
                    }
                }

                if changed.contains_key("PlaybackStatus") {
                    if let Ok(status) = self.playback_status().await {
                        yield PlayerEvent::PlaybackChanged(status);
                    }
                }
            }
        };

        Ok(Box::pin(output))
    }
}

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

    pub fn get_service(&self) -> OwnedWellKnownName {
        self.service.clone()
    }

    async fn proxy(&self) -> zbus::Result<PlayerProxy<'_>> {
        PlayerProxy::builder(&self.connection)
            .destination(&self.service)?
            .build()
            .await
    }

    pub async fn metadata(&self) -> zbus::Result<HashMap<String, OwnedValue>> {
        let proxy = self.proxy().await?;
        Ok(proxy.metadata().await?)
    }

    pub async fn get_current_position(&self) -> zbus::Result<Duration> {
        let proxy = self.proxy().await?;
        let micros = proxy.position().await?;
        Ok(Duration::microseconds(micros as i64))
    }

    pub async fn get_current_track(&self) -> zbus::Result<Track> {
        let metadata = self.proxy().await?.metadata().await?;

        Ok(Track::parse_track(metadata).await)
    }

    pub async fn get_playback_status(&self) -> zbus::Result<PlaybackStatus> {
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
            PlaybackCommand::Play => match self.get_playback_status().await? {
                PlaybackStatus::Playing => {}
                _ => proxy.play().await?,
            },
            PlaybackCommand::Pause => proxy.pause().await?,
            PlaybackCommand::Toggle => proxy.play_pause().await?,
            PlaybackCommand::Next => proxy.next().await?,
            PlaybackCommand::Previous => proxy.previous().await?,
            PlaybackCommand::Seek(offset) => {
                proxy.seek(offset.num_microseconds().unwrap_or(0)).await?
            }
            PlaybackCommand::SetPosition(position) => {
                let track = self.get_current_track().await?;
                let track_id = track
                    .id
                    .as_ref()
                    .ok_or_else(|| zbus::Error::Failure("missing track id".into()))?;

                proxy
                    .set_position(track_id, position.num_microseconds().unwrap_or(0))
                    .await?
            }
        }

        Ok(())
    }

    pub async fn events(&self) -> zbus::Result<impl Stream<Item = PlayerEvent>> {
        let properties_proxy = PropertiesProxy::builder(&self.connection)
            .destination(&self.service)?
            .path("/org/mpris/MediaPlayer2")?
            .build()
            .await?;

        let player_proxy = PlayerProxy::builder(&self.connection)
            .destination(&self.service)?
            .path("/org/mpris/MediaPlayer2")?
            .build()
            .await?;

        let mut properties = properties_proxy.receive_properties_changed().await?;
        let mut seeked = player_proxy.receive_seeked().await?;

        let output = stream! {
            loop {
                tokio::select! {
                    Some(signal) = properties.next() => {
                        let Ok(args) = signal.args() else {
                            continue;
                        };

                        let changed = args.changed_properties();

                        if changed.contains_key("Metadata") {
                        // Get the metadata here, create the track and then pass that through to
                        // avoid another call to get the new track?

                        // // print Metadata to see the info for if this is possible
                            yield PlayerEvent::TrackChanged;
                        }

                        if changed.contains_key("PlaybackStatus") {
                            yield PlayerEvent::PlaybackChanged;
                        }
                    }

                    Some(signal) = seeked.next() => {
                        let Ok(args) = signal.args() else {
                            continue;
                        };

                        let position = Duration::microseconds(*args.position());
                        yield PlayerEvent::Seeked(position);
                    }

                    else => break,
                }
            }
        };

        Ok(Box::pin(output))
    }
}

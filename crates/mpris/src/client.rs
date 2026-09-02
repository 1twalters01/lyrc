use std::collections::HashMap;

use async_stream::stream;
use chrono::Duration;
use futures_core::Stream;
use futures_util::StreamExt;
use zbus::{
    Connection,
    fdo::PropertiesProxy,
    names::OwnedWellKnownName,
    zvariant::{OwnedValue, Value},
};

use crate::{
    playback::{PlaybackCommand, PlaybackStatus, PlayerEvent},
    proxy::PlayerProxy,
    track::Track,
};

#[derive(Clone)]
pub struct MprisClient {
    connection: Connection,
    service: OwnedWellKnownName,
    player: String,
}

impl MprisClient {
    pub async fn find_players() -> zbus::Result<Vec<String>> {
        let connection = Connection::session().await?;

        let proxy = zbus::fdo::DBusProxy::new(&connection).await?;

        let names = proxy.list_names().await?;

        Ok(names
            .into_iter()
            .filter_map(|name| {
                name.strip_prefix("org.mpris.MediaPlayer2.")
                    .map(str::to_owned)
            })
            .collect::<Vec<_>>())
    }

    pub async fn choose_player(targets: &Vec<String>) -> zbus::Result<String> {
        let players = Self::find_players().await?;

        let mut clients = Vec::new();
        for player in &players {
            let client = Self::connect(player).await?;
            clients.push(client);
        }

        let mut playback_statuses = Vec::new();
        for client in clients {
            playback_statuses.push(client.get_playback_status().await?);
        }

        let playing_players: Vec<_> = players
            .iter()
            .enumerate()
            .filter(|(i, _)| playback_statuses[*i] == PlaybackStatus::Playing)
            .map(|(_, p)| p.clone())
            .collect();
        if playing_players.len() > 0 {
            for target in targets {
                if playing_players.contains(&String::from(target)) {
                    return Ok(String::from(target));
                }
            }

            return Ok(playing_players[0].clone());
        }

        let paused_players: Vec<_> = players
            .iter()
            .enumerate()
            .filter(|(i, _)| playback_statuses[*i] == PlaybackStatus::Paused)
            .map(|(_, c)| c)
            .cloned()
            .collect();
        if paused_players.len() > 0 {
            for target in targets {
                if paused_players.contains(&String::from(target)) {
                    return Ok(String::from(target));
                }
            }

            return Ok(paused_players[0].clone());
        }

        let stopped_players: Vec<_> = players
            .iter()
            .enumerate()
            .filter(|(i, _)| playback_statuses[*i] == PlaybackStatus::Stopped)
            .map(|(_, c)| c)
            .cloned()
            .collect();
        if stopped_players.len() > 0 {
            for target in targets {
                if stopped_players.contains(&String::from(target)) {
                    return Ok(String::from(target));
                }
            }

            return Ok(stopped_players[0].clone());
        }

        Ok(players[0].clone())
    }

    pub async fn connect(player: &str) -> zbus::Result<Self> {
        let connection = Connection::session().await?;
        let service = format!("org.mpris.MediaPlayer2.{player}");

        Ok(Self {
            connection,
            service: service.try_into()?,
            player: String::from(player),
        })
    }

    pub fn get_service(&self) -> OwnedWellKnownName {
        self.service.clone()
    }

    pub fn get_player(&self) -> String {
        self.player.clone()
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

        Ok(Track::parse_track(self, metadata).await)
    }

    pub async fn get_playback_status(&self) -> zbus::Result<PlaybackStatus> {
        let proxy = self.proxy().await?;
        let status = proxy.playback_status().await?;

        Ok(PlaybackStatus::parse(&status))
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

                        if let Some(Value::Dict(metadata)) = changed.get("Metadata") {
                            let owned_metadata: HashMap<String, OwnedValue> = metadata
                                .iter()
                                .filter_map(|(k, v)| {
                                    let key = match k {
                                        Value::Str(s) => s.to_string(),
                                        _ => return None,
                                    };
                                    let value = OwnedValue::try_from(v.clone()).ok()?;
                                    Some((key, value))
                                })
                                .collect();

                            let track = Track::parse_track(self, owned_metadata).await;
                            yield PlayerEvent::TrackChanged(track);
                        }

                        if let Some(Value::Str(status)) = changed.get("PlaybackStatus") {
                            let playback_status = PlaybackStatus::parse(status);
                            yield PlayerEvent::PlaybackChanged(playback_status);
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

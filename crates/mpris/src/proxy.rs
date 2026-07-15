use std::collections::HashMap;

use zbus::{proxy, zvariant::OwnedValue};

#[proxy(
    interface = "org.mpris.MediaPlayer2.Player",
    default_path = "/org/mpris/MediaPlayer2"
)]
pub trait Player {
    #[zbus(property)]
    fn playback_status(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn position(&self) -> zbus::Result<i64>;

    #[zbus(property)]
    fn metadata(&self) -> zbus::Result<HashMap<String, OwnedValue>>;

    #[zbus(signal)]
    fn properties_changed(
        &self,
        interface_name: &str,
        changed_properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        invalidated_properties: Vec<String>,
    ) -> zbus::Result<()>;

    fn play(&self) -> zbus::Result<()>;

    fn pause(&self) -> zbus::Result<()>;

    fn play_pause(&self) -> zbus::Result<()>;

    fn next(&self) -> zbus::Result<()>;

    fn previous(&self) -> zbus::Result<()>;

    fn seek(&self, offset: i64) -> zbus::Result<()>;

    fn set_position(&self, offset: i64) -> zbus::Result<()>;
}

use zbus::Connection;

pub struct Player {
    connection: Connection,
}

impl Player {
    pub async fn connect() -> zbus::Result<Self> {
        let connection = Connection::session().await?;

        Ok(Self { connection })
    }
}

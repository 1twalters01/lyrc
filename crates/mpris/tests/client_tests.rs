use chrono::Duration;
use mpris::{client::MprisClient, playback::{PlaybackCommand, PlaybackStatus}, track::Track};

#[tokio::test]
async fn connect_to_cmus() {
    let player = "cmus";
    let client = MprisClient::connect(player)
        .await
        .expect("failed to connect to cmus");

    assert_eq!(client.get_service().as_str(), "org.mpris.MediaPlayer2.cmus");
}

#[tokio::test]
async fn get_current_position() {
    let player = "cmus";
    let client = MprisClient::connect(player)
        .await
        .expect("failed to connect to cmus");

    let current_position = client.get_current_position().await.unwrap();
    assert_ne!(current_position, chrono::Duration::microseconds(5));
}

#[tokio::test]
#[ignore = "takes 2 seconds"]
async fn poll_current_position() {
    let client = MprisClient::connect("cmus")
        .await
        .expect("failed to connect to cmus");

    for _ in 0..20 {
        let position = client
            .get_current_position()
            .await
            .expect("failed to get position");

        println!(
            "Position: {}s {}µs",
            position.num_seconds(),
            position.num_microseconds().unwrap()
        );

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

#[tokio::test]
async fn get_current_track() {
    let player = "cmus";
    let client = MprisClient::connect(player)
        .await
        .expect("failed to connect to cmus");

    let current_track = client.get_current_track().await.unwrap();

    assert_ne!(current_track.title, String::new());
}

#[tokio::test]
async fn get_playback_status() {
    let player = "cmus";
    let client = MprisClient::connect(player)
        .await
        .expect("failed to connect to cmus");

    let playback_status = client.get_playback_status().await.unwrap();

    assert_eq!(playback_status, PlaybackStatus::Playing);
}

#[tokio::test]
#[ignore = "changes current track"]
async fn execute_next_track() {
    let player = "cmus";
    let client = MprisClient::connect(player)
        .await
        .expect("failed to connect to cmus");

    let current_track = client.get_current_track().await.unwrap();
    client.execute(PlaybackCommand::Next).await.unwrap();
    let next_track = client.get_current_track().await.unwrap();

    assert_eq!(
        current_track.track_number.and_then(|n| Some(n + 1)),
        next_track.track_number
    );


#[tokio::test]
#[ignore = "changes current track"]
async fn execute_previous_track() {
    let player = "cmus";
    let client = MprisClient::connect(player)
        .await
        .expect("failed to connect to cmus");

    let current_track = client.get_current_track().await.unwrap();
    client.execute(PlaybackCommand::Previous).await.unwrap();
    let next_track = client.get_current_track().await.unwrap();

    assert_eq!(
        current_track.track_number.and_then(|n| Some(n - 1)),
        next_track.track_number
    );
}}

#[tokio::test]
async fn execute_seek_time() {
    let player = "cmus";
    let client = MprisClient::connect(player)
        .await
        .expect("failed to connect to cmus");

    let target_position = Duration::seconds(10) + Duration::microseconds(251000);

    let current_position = client.get_current_position().await.unwrap();
    assert_ne!(current_position, target_position);

    client.execute(PlaybackCommand::Seek(target_position)).await.unwrap();

    let current_position = client.get_current_position().await.unwrap();
    assert_eq!(current_position, target_position);
}

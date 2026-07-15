use mpris::{client::MprisClient, track::Track};

#[tokio::test]
async fn connect_to_cmus() {
    let player = "cmus";
    let client = MprisClient::connect(player)
        .await
        .expect("failed to connect to cmus");

    assert_eq!(client.get_service().as_str(), "org.mpris.MediaPlayer2.cmus");
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

use chrono::Duration;

pub struct Config {
    pub rewind_duration: Duration,
    pub fast_forward_duration: Duration,
    pub forwards_cue_increment: Duration,
    pub backwards_cue_increment: Duration,
}

use chrono::Duration;

pub struct Config {
    pub rewind_duration: Duration,
    pub fast_forward_duration: Duration,
    pub forwards_cue_increment_large: Duration,
    pub backwards_cue_increment_large: Duration,
    pub forwards_cue_increment_small: Duration,
    pub backwards_cue_increment_small: Duration,
}

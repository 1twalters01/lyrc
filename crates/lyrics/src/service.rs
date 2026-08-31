use std::collections::HashMap;

use crate::{provider::LyricsDownloader, providers::lrclib::LrclibProvider};

pub struct LyricsService {
    pub providers: HashMap<String, Box<dyn LyricsDownloader>>,
}

impl Default for LyricsService {
    fn default() -> Self {
        Self {
            providers: HashMap::from([(
                String::from("lrclib"),
                Box::new(LrclibProvider) as Box<dyn LyricsDownloader>,
            )]),
        }
    }
}

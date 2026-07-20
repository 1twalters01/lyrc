use std::collections::HashMap;

use crate::provider::LyricsProvider;

pub struct LyricsService {
    providers: HashMap<String, Box<dyn LyricsProvider>>,
}

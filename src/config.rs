use crate::{leagues::League, Sportsbook};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    sportsbooks: HashMap<Sportsbook, SportsbookSettings>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_url(&self, sportsbook: Sportsbook, league: League) -> String {
        let settings = self.sportsbooks.get(&sportsbook).unwrap();
        let url = match league {
            League::MLB => &settings.mlb,
        };
        url.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        let config = include_str!("../stormyscraper.json");
        serde_json::from_str(config).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SportsbookSettings {
    /// URL of MLB page
    mlb: String,
}

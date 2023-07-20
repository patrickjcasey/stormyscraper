/// Supported Sportsbooks
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Sportsbook {
    Draftkings,
}

/// Supported leagues for scraping
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum League {
    MLB,
}

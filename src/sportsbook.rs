use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
/// Supported Sportsbooks for scraping
pub enum Sportsbook {
    Draftkings,
}

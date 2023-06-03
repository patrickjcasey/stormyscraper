use crate::odds_types::{moneyline::Moneyline, over_under::OverUnder, spread::Spread};

pub struct MlbGame {
    home_team: String,
    away_team: String,
    over_under: OverUnder,
    spread: Spread,
    moneyline: Moneyline,
}

impl MlbGame {
    pub fn new() -> Self {
        Self {
            home_team: "".to_string(),
            away_team: "".to_string(),
            over_under: OverUnder::new(),
            spread: Spread::new(),
            moneyline: Moneyline::new(),
        }
    }
}

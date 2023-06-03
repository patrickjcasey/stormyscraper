// This file implements the structure of a Moneyline bet

pub struct Spread {
    pub home_team_odds: i32,
    pub home_team_line: f32,
    pub away_team_odds: i32,
    pub away_team_line: f32,
}

impl Spread {
    pub fn new() -> Self {
        Self {
            home_team_odds: 0,
            home_team_line: 0.0,
            away_team_odds: 0,
            away_team_line: 0.0,
        }
    }

    pub fn set_home_team_odds(mut self, home_team_odds: i32) -> Self {
        self.home_team_odds = home_team_odds;
        self
    }

    pub fn set_home_team_line(mut self, home_team_line: f32) -> Self {
        self.home_team_line = home_team_line;
        self
    }

    pub fn set_away_team_odds(mut self, away_team_odds: i32) -> Self {
        self.away_team_odds = away_team_odds;
        self
    }

    pub fn set_away_team_line(mut self, away_team_line: f32) -> Self {
        self.away_team_line = away_team_line;
        self
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn spread_works() {
        use crate::odds_types::spread::Spread;
        let mut spread = Spread::new();
        assert_eq!(spread.away_team_odds, 0);
        assert_eq!(spread.away_team_line, 0.0);
        assert_eq!(spread.home_team_odds, 0);
        assert_eq!(spread.away_team_line, 0.0);
        spread = spread
            .set_home_team_odds(-100)
            .set_home_team_line(5.5)
            .set_away_team_odds(100)
            .set_away_team_line(-5.5);
        assert_eq!(spread.away_team_odds, 100);
        assert_eq!(spread.away_team_line, -5.5);
        assert_eq!(spread.home_team_odds, -100);
        assert_eq!(spread.home_team_line, 5.5);
    }
}

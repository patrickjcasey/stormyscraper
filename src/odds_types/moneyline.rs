// This file implements the structure of a Moneyline bet

pub struct Moneyline {
    pub home_team_odds: i32,
    pub away_team_odds: i32,
}

impl Moneyline {
    pub fn new() -> Self {
        Self {
            home_team_odds: 0,
            away_team_odds: 0,
        }
    }

    pub fn set_home_team_odds(mut self, home_team_odds: i32) -> Self {
        self.home_team_odds = home_team_odds;
        self
    }

    pub fn set_away_team_odds(mut self, away_team_odds: i32) -> Self {
        self.away_team_odds = away_team_odds;
        self
    }
}       

#[cfg(test)]
mod tests {
    #[test]
    fn moneyline_works() {
        use crate::odds_types::moneyline::Moneyline;
        let mut moneyline = Moneyline::new();
        assert_eq!(moneyline.away_team_odds, 0);
        assert_eq!(moneyline.home_team_odds, 0);
        moneyline = moneyline.set_home_team_odds(-100).set_away_team_odds(100);
        assert_eq!(moneyline.away_team_odds, 100);
        assert_eq!(moneyline.home_team_odds, -100);
    }
}

// This file contains all of the logic needed to hold Spread for a team game

#[derive(Debug, Default, Clone)]
pub struct Spread {
    pub home_team_odds: i32,
    pub home_team_line: f32,
    pub away_team_odds: i32,
    pub away_team_line: f32,
}

impl Spread {
    pub fn new() -> Self {
        Self::default()
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
    use super::*;
    #[test]
    fn spread_works() {
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

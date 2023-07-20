
#[derive(Debug, Default, Clone)]
pub struct OverUnder {
    pub over_odds: i32,
    pub over_line: f32,
    pub under_odds: i32,
    pub under_line: f32,
}

impl OverUnder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_over_odds(mut self, over_odds: i32) -> Self {
        self.over_odds = over_odds;
        self
    }

    pub fn set_over_line(mut self, over_line: f32) -> Self {
        self.over_line = over_line;
        self
    }

    pub fn set_under_odds(mut self, under_odds: i32) -> Self {
        self.under_odds = under_odds;
        self
    }

    pub fn set_under_line(mut self, under_line: f32) -> Self {
        self.under_line = under_line;
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*; 
    #[test]
    fn over_under_works() {
        let mut over_under = OverUnder::new();
        assert_eq!(over_under.over_odds, 0);
        assert_eq!(over_under.over_line, 0.0);
        assert_eq!(over_under.under_odds, 0);
        assert_eq!(over_under.under_line, 0.0);
        over_under = over_under
            .set_over_odds(-100)
            .set_over_line(5.5)
            .set_under_odds(100)
            .set_under_line(5.5);
        assert_eq!(over_under.over_odds, -100);
        assert_eq!(over_under.over_line, 5.5);
        assert_eq!(over_under.under_odds, 100);
        assert_eq!(over_under.under_line, 5.5);
    }
}

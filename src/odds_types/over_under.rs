use crate::game_types::TeamGameEntry;

#[derive(Clone)]
pub struct OverUnder {
    pub over_odds: i32,
    pub under_odds: i32,
    pub over_line: f32,
    pub under_line: f32,
}

impl OverUnder {
    pub fn new(home_entry: &TeamGameEntry, away_entry: &TeamGameEntry) -> Option<Self> {
        match (
            &home_entry.over_under_line,
            &home_entry.over_under_odds,
            &away_entry.over_under_line,
            &away_entry.over_under_odds,
        ) {
            (Some(over_line), Some(over_odds), Some(under_line), Some(under_odds)) => Some(Self {
                over_line: over_line
                    .chars()
                    .map(|ch| {
                        if ch == char::from_u32(0x2212).unwrap() {
                            '-'
                        } else {
                            ch
                        }
                    })
                    .collect::<String>()
                    .parse()
                    .unwrap(),
                under_line: under_line
                    .chars()
                    .map(|ch| {
                        if ch == char::from_u32(0x2212).unwrap() {
                            '-'
                        } else {
                            ch
                        }
                    })
                    .collect::<String>()
                    .parse()
                    .unwrap(),
                over_odds: over_odds
                    .chars()
                    .map(|ch| {
                        if ch == char::from_u32(0x2212).unwrap() {
                            '-'
                        } else {
                            ch
                        }
                    })
                    .collect::<String>()
                    .parse()
                    .unwrap(),
                under_odds: under_odds
                    .chars()
                    .map(|ch| {
                        if ch == char::from_u32(0x2212).unwrap() {
                            '-'
                        } else {
                            ch
                        }
                    })
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            }),
            _ => None,
        }
    }
    pub fn over_as_string(&self) -> String {
        format!("O {}  {}", self.over_line, self.over_odds)
    }

    pub fn under_as_string(&self) -> String {
        format!("U {}  {}", self.under_line, self.under_odds)
    }
}

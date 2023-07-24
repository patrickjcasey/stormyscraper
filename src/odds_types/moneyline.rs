use crate::game_types::TeamGameEntry;

#[derive(Clone)]
pub struct Moneyline {
    pub home_team_odds: i32,
    pub away_team_odds: i32,
}

impl Moneyline {
    pub fn new(home_entry: &TeamGameEntry, away_entry: &TeamGameEntry) -> Option<Self> {
        match (&home_entry.moneyline, &away_entry.moneyline) {
            (Some(home_moneyline), Some(away_moneyline)) => Some(Self {
                home_team_odds: home_moneyline
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
                away_team_odds: away_moneyline
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

    pub fn home_moneyline_as_string(&self) -> String {
        format!("{}", self.home_team_odds)
    }

    pub fn away_moneyline_as_string(&self) -> String {
        format!("{}", self.away_team_odds)
    }
}

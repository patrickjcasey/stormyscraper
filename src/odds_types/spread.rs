// This file contains all of the logic needed to hold Spread for a team game

use crate::game_types::TeamGameEntry;

#[derive(Clone)]
pub struct Spread {
    pub home_team_line: f32,
    pub away_team_line: f32,
    pub home_team_odds: i32,
    pub away_team_odds: i32,
}

impl Spread {
    pub fn new(home_entry: &TeamGameEntry, away_entry: &TeamGameEntry) -> Option<Self> {
        println!("home: {:?}", home_entry.spread_odds);
        println!("away: {:?}", away_entry.spread_odds);
        match (
            &home_entry.spread_line,
            &home_entry.spread_odds,
            &away_entry.spread_line,
            &away_entry.spread_odds,
        ) {
            (
                Some(home_team_line),
                Some(home_team_odds),
                Some(away_team_line),
                Some(away_team_odds),
            ) => Some(Self {
                // TODO write a common function for this replacement of 0x2212 (html minus sign)
                home_team_line: home_team_line
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
                away_team_line: away_team_line
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
                home_team_odds: home_team_odds
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
                away_team_odds: away_team_odds
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
    pub fn home_spread_as_string(&self) -> String {
        format!("{}  {}", self.home_team_line, self.home_team_odds)
    }

    pub fn away_spread_as_string(&self) -> String {
        format!("{}  {}", self.away_team_line, self.away_team_odds)
    }
}

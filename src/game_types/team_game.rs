use std::fmt::Debug;

use crate::leagues::{League, LeagueTeam};
use crate::odds_types::{Moneyline, OverUnder, Spread};
use scraper::{ElementRef, Selector};

#[derive(Clone)]
pub struct TeamGame {
    pub home_team: LeagueTeam,
    pub away_team: LeagueTeam,
    pub over_under: Option<OverUnder>,
    pub spread: Option<Spread>,
    pub moneyline: Option<Moneyline>,
}

impl TeamGame {
    pub fn new(home_entry: &TeamGameEntry, away_entry: &TeamGameEntry) -> Self {
        Self {
            home_team: home_entry.team,
            away_team: away_entry.team,
            // TODO
            over_under: OverUnder::new(home_entry, away_entry),
            spread: Spread::new(home_entry, away_entry),
            moneyline: Moneyline::new(home_entry, away_entry),
        }
    }

    pub fn get_home_team(&self) -> String {
        self.home_team.to_string()
    }

    pub fn get_away_team(&self) -> String {
        self.away_team.to_string()
    }

    pub fn get_home_spread(&self) -> String {
        match &self.spread {
            Some(x) => x.home_spread_as_string(),
            None => "None".to_string(),
        }
    }

    pub fn get_away_spread(&self) -> String {
        match &self.spread {
            Some(x) => x.away_spread_as_string(),
            None => "None".to_string(),
        }
    }

    pub fn get_over(&self) -> String {
        match &self.over_under {
            Some(x) => x.over_as_string(),
            None => "None".to_string(),
        }
    }

    pub fn get_under(&self) -> String {
        match &self.over_under {
            Some(x) => x.under_as_string(),
            None => "None".to_string(),
        }
    }

    pub fn get_home_moneyline(&self) -> String {
        match &self.moneyline {
            Some(x) => x.home_moneyline_as_string(),
            None => "None".to_string(),
        }
    }

    pub fn get_away_moneyline(&self) -> String {
        match &self.moneyline {
            Some(x) => x.away_moneyline_as_string(),
            None => "None".to_string(),
        }
    }
}

impl Debug for TeamGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "----------------------").unwrap();
        writeln!(
            f,
            "| {} | {} | {} | {} |",
            self.get_home_team(),
            self.get_home_spread(),
            self.get_over(),
            self.get_home_moneyline()
        )
        .unwrap();
        writeln!(f, "----------------------").unwrap();
        writeln!(
            f,
            "| {} | {} | {} | {} |",
            self.get_away_team(),
            self.get_away_spread(),
            self.get_under(),
            self.get_away_moneyline()
        )
        .unwrap();
        writeln!(f, "----------------------").unwrap();
        Ok(())
    }
}

// TODO move this
#[derive(Clone, Debug)]
pub struct TeamGameEntry {
    pub team: LeagueTeam,
    pub spread_odds: Option<String>,
    pub spread_line: Option<String>,
    pub over_under_odds: Option<String>,
    pub over_under_line: Option<String>,
    pub moneyline: Option<String>,
}

#[derive(Debug)]
pub enum EntryDecodeError {
    TeamNameDecodeError,
}

impl TeamGameEntry {
    pub fn extract_entry(league: League, value: ElementRef) -> Result<Self, EntryDecodeError> {
        let team_as_string: String = value
            .select(&Selector::parse("div.event-cell__name-text").unwrap())
            .next()
            .unwrap()
            .text()
            .collect();
        let team = LeagueTeam::from_str(league, &team_as_string[..])
            .map_err(|_| EntryDecodeError::TeamNameDecodeError)?;

        let mut spread_line = None;
        if let Some(x) = value
            .select(&Selector::parse("span.sportsbook-outcome-cell__line").unwrap())
            .next()
        {
            spread_line = Some(x.text().collect())
        }

        let mut over_under_line = None;
        if let Some(x) = value
            .select(&Selector::parse("span.sportsbook-outcome-cell__line").unwrap())
            .nth(1)
        {
            over_under_line = Some(x.text().collect())
        }

        // TODO determine correct CSS selector for this field
        // let moneyline_line = value.select(&Selector::parse("span.sportsbook-odds american no-margin default-color").unwrap()).nth(0).unwrap().text().collect();
        let mut spread_odds = None;
        if let Some(x) = value
            .select(&Selector::parse("span.sportsbook-odds").unwrap())
            .next()
        {
            spread_odds = Some(x.text().collect())
        }
        let mut over_under_odds = None;
        if let Some(x) = value
            .select(&Selector::parse("span.sportsbook-odds").unwrap())
            .nth(1)
        {
            over_under_odds = Some(x.text().collect())
        }
        let mut moneyline = None;
        if let Some(x) = value
            .select(&Selector::parse("span.sportsbook-odds").unwrap())
            .nth(2)
        {
            moneyline = Some(x.text().collect())
        }
        Ok(Self {
            team,
            spread_odds,
            spread_line,
            moneyline,
            over_under_odds,
            over_under_line,
        })
    }
}

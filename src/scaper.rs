use crate::game_types::{TeamGame, TeamGameEntry};
use crate::leagues::{League, LeagueTeam};
use crate::Config;
use crate::Sportsbook;
use scraper::{ElementRef, Html, Selector};
use std::fmt::Error;

#[derive(Debug)]
pub enum ScraperError {
    FetchUrlError,
}

pub struct StormyScraper {
    config: Config,
}

impl StormyScraper {
    pub fn new() -> Self {
        let config = Config::new();
        StormyScraper { config }
    }

    pub fn scrape_today_games(
        &self,
        sportsbook: Sportsbook,
        league: League,
    ) -> Result<Vec<TeamGame>, ScraperError> {
        let url = self.config.get_url(sportsbook, league);
        let data = fetch_url(&url).map_err(|_| ScraperError::FetchUrlError)?;
        let fragment = Html::parse_document(&data);
        let selector = Selector::parse("tbody.sportsbook-table__body").unwrap();
        // Today's games are stored in the first instance of tbody.sportsbook-table__body

        let todays_games = fragment.select(&selector).next().unwrap();

        let entry = Selector::parse("tr").unwrap();

        let mut entries: Vec<TeamGameEntry> = vec![];
        for game in todays_games.select(&entry) {
            entries.push(TeamGameEntry::extract_entry(league, game).unwrap());
        }
        println!("Completed gathering games!");
        let mut games = vec![];
        for entry in entries.chunks(2) {
            games.push(TeamGame::new(&entry[1], &entry[0]));
        }
        Ok(games)
    }
}

fn fetch_url(url: &str) -> Result<String, Error> {
    match reqwest::blocking::get(url) {
        Ok(response) => Ok(response.text().unwrap()),
        Err(_) => {
            eprintln!("Error retrieving: {}", url);
            Err(Error)
        }
    }
}

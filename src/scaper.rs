use crate::game_types::{TeamGame, TeamGameEntry};
use crate::leagues::League;
use crate::Config;
use crate::Sportsbook;
use reqwest::ClientBuilder;
use scraper::{Html, Selector};
use std::time::Duration;

#[derive(Debug)]
pub enum ScraperError {
    /// occurs when unable to get data from URL
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

    pub async fn scrape_today_games(
        &self,
        sportsbook: Sportsbook,
        league: League,
    ) -> Result<Vec<TeamGame>, ScraperError> {
        let url = self.config.get_url(sportsbook, league);
        let data = fetch_url(&url)
            .await
            .map_err(|_| ScraperError::FetchUrlError)?;
        let fragment = Html::parse_document(&data);
        let selector = Selector::parse("tbody.sportsbook-table__body").unwrap();
        // Today's games are stored in the first instance of tbody.sportsbook-table__body
        let todays_games = fragment.select(&selector).nth(1).unwrap();
        let entry = Selector::parse("tr").unwrap();

        // create a TeamGameEntry for each "tr" in the table
        let entries: Vec<TeamGameEntry> = todays_games
            .select(&entry)
            .into_iter()
            .map(|x| TeamGameEntry::extract_entry(league, x).unwrap())
            .collect();
        println!("Completed gathering games!");
        let mut games = vec![];
        for entry in entries.chunks(2) {
            games.push(TeamGame::new(&entry[1], &entry[0]));
        }
        println!("{}", games.len());
        Ok(games)
    }

    pub fn push_odds_to_db(games: Vec<TeamGame>) {}
}

// TODO: normalize the returned HTTP (replace HTML - with actual - ....)
async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let request = ClientBuilder::new().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36").timeout(Duration::from_secs(10)).build()?;
    match request.get(url).send().await {
        Ok(response) => Ok(response.text().await.unwrap()),
        Err(e) => {
            eprintln!("Error retrieving: {}", url);
            Err(e)
        }
    }
}

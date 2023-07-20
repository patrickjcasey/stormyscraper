use scraper::{ElementRef, Html, Selector};
use std::fmt::Error;
use stormyconfig::Config;
use stormycore::{League, Sportsbook};
use stormyevent::TeamGame;

#[derive(Debug)]
pub enum ScraperError {
    FetchUrlError,
}

pub struct StormyScraper {
    config: Config,
}

// TODO move this
#[derive(Clone, Debug)]
pub struct Entry {
    team: String,
    spread_odds: Option<String>,
    spread_line: Option<String>,
    over_under_odds: Option<String>,
    over_under_line: Option<String>,
    moneyline: Option<String>,
}

#[derive(Debug)]
pub struct RawGame {
    pub home_entry: Entry,
    pub away_entry: Entry,
}

impl From<ElementRef<'_>> for Entry {
    fn from(value: ElementRef) -> Self {
        let team = value
            .select(&Selector::parse("div.event-cell__name-text").unwrap())
            .next()
            .unwrap()
            .text()
            .collect();
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
        Self {
            team,
            spread_odds,
            spread_line,
            moneyline,
            over_under_odds,
            over_under_line,
        }
    }
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
    ) -> Result<Vec<RawGame>, ScraperError> {
        let url = self.config.get_url(sportsbook, league);
        // use question mark operator
        let data = fetch_url(&url)
            .map_err(|_| ScraperError::FetchUrlError)
            .unwrap();
        let fragment = Html::parse_document(&data);
        let selector = Selector::parse("tbody.sportsbook-table__body").unwrap();
        // Today's games are stored in the first instance of tbody.sportsbook-table__body

        let todays_games = fragment.select(&selector).next().unwrap();

        let entry = Selector::parse("tr").unwrap();

        let mut entries: Vec<Entry> = vec![];
        for game in todays_games.select(&entry) {
            // let team = game
            //     .select(&Selector::parse("div.event-cell__name-text").unwrap())
            //     .next()
            //     .unwrap();
            // let moneyline_odds = game
            //     .select(&Selector::parse("span.sportsbook-outcome-cell__line").unwrap())
            //     .next()
            //     .unwrap();
            entries.push(Entry::from(game));
        }
        println!("Completed gathering games!");
        let mut games = vec![];
        for entry in entries.chunks(2) {
            games.push(RawGame {
                away_entry: entry[0].clone(),
                home_entry: entry[1].clone(),
            });
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

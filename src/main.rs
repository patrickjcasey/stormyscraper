mod games;
mod endpoints;
mod odds_types;
mod fetch_url;

use reqwest;
use scraper::{Html, Selector, ElementRef};

use crate::{endpoints::draftkings::DraftkingsEndpoints, fetch_url::fetch_url, games::mlb::MlbGame, odds_types::moneyline};

#[derive(Debug)]
struct Entry {
    team: String,
    spread_odds: String,
    spread_line: String,
    over_under_odds: String,
    over_under_line: String,
    moneyline: String,
}

impl From<ElementRef<'_>> for Entry {
    fn from(value: ElementRef) -> Self {
        let team = value.select(&Selector::parse("div.event-cell__name-text").unwrap()).next().unwrap().text().collect();
        let spread_line = value.select(&Selector::parse("span.sportsbook-outcome-cell__line").unwrap()).nth(0).unwrap().text().collect();
        let over_under_line = value.select(&Selector::parse("span.sportsbook-outcome-cell__line").unwrap()).nth(1).unwrap().text().collect();

        // TODO determine correct CSS selector for this field
        // let moneyline_line = value.select(&Selector::parse("span.sportsbook-odds american no-margin default-color").unwrap()).nth(0).unwrap().text().collect();  
        let spread_odds = value.select(&Selector::parse("span.sportsbook-odds").unwrap()).nth(0).unwrap().text().collect();
        let over_under_odds = value.select(&Selector::parse("span.sportsbook-odds").unwrap()).nth(1).unwrap().text().collect();
        let moneyline = value.select(&Selector::parse("span.sportsbook-odds").unwrap()).nth(2).unwrap().text().collect();
        Self {
            team: team,
            spread_odds: spread_odds,
            spread_line: spread_line,
            moneyline: moneyline,
            over_under_odds: over_under_odds,
            over_under_line: over_under_line,
        }


    }
}

fn main() {
    let endpoints = DraftkingsEndpoints::default();
    let dk_data = fetch_url(&endpoints.wnba);
    let fragment = Html::parse_document(&dk_data.unwrap());
    let selector = Selector::parse("tbody.sportsbook-table__body").unwrap();
    // Today's games are stored in the first instance of tbody.sportsbook-table__body
    
    let todays_games = fragment.select(&selector).next().unwrap();

    let entry = Selector::parse("tr").unwrap();

    let mut live_games: Vec<MlbGame> = vec![];

    let entries: Vec<Entry> = todays_games.select(&entry).map(Entry::from).collect();

    for game in todays_games.select(&entry) {
        let team = game.select(&Selector::parse("div.event-cell__name-text").unwrap()).next().unwrap();
        println!("{}", team.text().next().unwrap());
        let moneyline_odds = game.select(&Selector::parse("span.sportsbook-outcome-cell__line").unwrap()).next().unwrap();
        println!("{}", moneyline_odds.text().next().unwrap());
        let entry = Entry::from(game);
        println!("{:#?}", entry);
        println!("------------------")
    }

    println!("Completed gathering games!");


}

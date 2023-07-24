use stormyscraper::leagues::League;
use stormyscraper::{Sportsbook, StormyScraper};

fn main() {
    let scraper = StormyScraper::new();
    let games = scraper
        .scrape_today_games(Sportsbook::Draftkings, League::MLB)
        .unwrap();
    println!("{:#?}", games);
}

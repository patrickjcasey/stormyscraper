use stormycore::{League, Sportsbook};
use stormyscraper::StormyScraper;

fn main() {
    let scraper = StormyScraper::new();
    let games = scraper.scrape_today_games(Sportsbook::Draftkings, League::MLB);
    println!("{:#?}", games);
}

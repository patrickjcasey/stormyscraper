use stormyscraper::leagues::League;
use stormyscraper::{Sportsbook, StormyScraper};

#[tokio::main]
async fn main() {
    let scraper = StormyScraper::new();
    let games = scraper
        .scrape_today_games(Sportsbook::Draftkings, League::MLB)
        .await
        .unwrap();
    println!("{:#?}", games);
}

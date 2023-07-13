// This file contains all of the logic needed to fetch a URL from an oddsmaker

use std::fmt::Error;


pub fn fetch_url(url: &str) -> Result<String, Error> {
    match reqwest::blocking::get(url) {
        Ok(response) => Ok(response.text().unwrap()),
        Err(_) => {
            eprintln!("Error retrieving: {}", url);
            Err(Error)
        }
    }
}

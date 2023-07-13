// This file contains all of the draftkings endpoints
// TODO: at some point it would be useful to be able to update this with some sort of CLI via some
// admin interface! These may be better of being stored in a DB idk
//

#[derive(Debug)]
pub struct DraftkingsEndpoints {
    pub mlb: String,
    pub wnba: String
}


impl Default for DraftkingsEndpoints {
    fn default() -> Self {
        Self { mlb: "https://sportsbook.draftkings.com/leagues/baseball/mlb".to_string(),
            wnba: "https://sportsbook.draftkings.com/leagues/basketball/wnba".to_string(),
        }
    }
}

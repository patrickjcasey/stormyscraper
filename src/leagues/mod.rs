use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::TeamDecodeError;

use self::mlb::MlbTeam;

pub mod mlb;

/// Supported leagues for scraping
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum League {
    MLB,
}

#[derive(Debug, Clone, Copy)]
/// enum containing leagues and all of their respective teams
pub enum LeagueTeam {
    Mlb(MlbTeam),
}

impl LeagueTeam {
    pub fn from_str(league: League, team_name: &str) -> Result<LeagueTeam, TeamDecodeError> {
        match league {
            League::MLB => Ok(LeagueTeam::Mlb(MlbTeam::try_from(team_name)?)),
        }
    }
}

impl Display for LeagueTeam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let team = match self {
            Self::Mlb(team) => team.to_string(),
        };
        write!(f, "{}", team).unwrap();
        Ok(())
    }
}

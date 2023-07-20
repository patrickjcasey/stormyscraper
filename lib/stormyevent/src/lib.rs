// This contains all of the logic to deal with MLB teams
use stormyodds::{Moneyline, OverUnder, Spread};

#[derive(Debug, Default, Clone)]
pub enum MlbTeam {
    ArizonaDiamondbacks,
    AtlantaBraves,
    BaltimoreOrioles,
    #[default]
    BostonRedSox,
    ChicagoWhiteSox,
    ChicagoCubs,
    CincinnatiReds,
    ClevelandGuardians,
    ColoradoRockies,
    DetroitTigers,
    HoustonAstros,
    KansasCityRoyals,
    LosAngelesAngels,
    LosAngelesDodgers,
    MiamiMarlins,
    MilwaukeeBrewers,
    MinnesotaTwins,
    NewYorkMets,
    NewYorkYankees,
    OaklandAthletics,
    PhiladelhiaPhillies,
    PittsburghPirates,
    SanDiegoPadres,
    SanFrancisoGiants,
    SeattleMariners,
    StLouisCardinals,
    TampaBayRays,
    TexasRangers,
    TorontoBlueJays,
    WashingtonNationals,
}

#[derive(Debug, Default, Clone)]
pub struct TeamGame {
    home_team: MlbTeam,
    away_team: MlbTeam,
    over_under: OverUnder,
    spread: Spread,
    moneyline: Moneyline,
}

impl TeamGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_home_team(mut self, home_team: MlbTeam) -> Self {
        self.home_team = home_team;
        self
    }

    pub fn set_away_team(mut self, away_team: MlbTeam) -> Self {
        self.away_team = away_team;
        self
    }

    pub fn set_over_under(mut self, over_under: OverUnder) -> Self {
        self.over_under = over_under;
        self
    }

    pub fn set_spread(mut self, spread: Spread) -> Self {
        self.spread = spread;
        self
    }

    pub fn set_moneyline(mut self, moneyline: Moneyline) -> Self {
        self.moneyline = moneyline;
        self
    }
}

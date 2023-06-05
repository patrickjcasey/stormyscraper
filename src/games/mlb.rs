use crate::odds_types::{moneyline::Moneyline, over_under::OverUnder, spread::Spread};

pub enum MlbTeam {
    ArizonaDiamondbacks,
    AtlantaBraves,
    BaltimoreOrioles,
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

pub struct MlbGame {
    home_team: MlbTeam,
    away_team: MlbTeam,
    over_under: OverUnder,
    spread: Spread,
    moneyline: Moneyline,
}

impl MlbGame {
    pub fn new() -> Self {
        Self {
            home_team: MlbTeam::BostonRedSox, 
            away_team: MlbTeam::NewYorkYankees, 
            over_under: OverUnder::new(),
            spread: Spread::new(),
            moneyline: Moneyline::new(),
        }
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

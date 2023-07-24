// This file contains all of the MlbTeams
use crate::errors::TeamDecodeError;
use strum::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Display)]
/// enum of all MLB teams
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

impl TryFrom<&str> for MlbTeam {
    type Error = TeamDecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            x if x.contains("diamondbacks") => Ok(Self::ArizonaDiamondbacks),
            x if x.contains("braves") => Ok(Self::AtlantaBraves),
            x if x.contains("orioles") => Ok(Self::BaltimoreOrioles),
            x if x.contains("red sox") => Ok(Self::BostonRedSox),
            x if x.contains("white sox") => Ok(Self::ChicagoWhiteSox),
            x if x.contains("cubs") => Ok(Self::ChicagoCubs),
            x if x.contains("reds") => Ok(Self::CincinnatiReds),
            x if x.contains("guardians") => Ok(Self::ClevelandGuardians),
            x if x.contains("rockies") => Ok(Self::ColoradoRockies),
            x if x.contains("tigers") => Ok(Self::DetroitTigers),
            x if x.contains("astros") => Ok(Self::HoustonAstros),
            x if x.contains("royals") => Ok(Self::KansasCityRoyals),
            x if x.contains("angels") => Ok(Self::LosAngelesAngels),
            x if x.contains("dodgers") => Ok(Self::LosAngelesDodgers),
            x if x.contains("marlins") => Ok(Self::MiamiMarlins),
            x if x.contains("brewers") => Ok(Self::MilwaukeeBrewers),
            x if x.contains("twins") => Ok(Self::MinnesotaTwins),
            x if x.contains("mets") => Ok(Self::NewYorkMets),
            x if x.contains("yankees") => Ok(Self::NewYorkYankees),
            x if x.contains("athletics") => Ok(Self::OaklandAthletics),
            x if x.contains("phillies") => Ok(Self::PhiladelhiaPhillies),
            x if x.contains("pirates") => Ok(Self::PittsburghPirates),
            x if x.contains("padres") => Ok(Self::SanDiegoPadres),
            x if x.contains("giants") => Ok(Self::SanFrancisoGiants),
            x if x.contains("mariners") => Ok(Self::SeattleMariners),
            x if x.contains("cardinals") => Ok(Self::StLouisCardinals),
            x if x.contains("rays") => Ok(Self::TampaBayRays),
            x if x.contains("rangers") => Ok(Self::TexasRangers),
            x if x.contains("blue jays") => Ok(Self::TorontoBlueJays),
            x if x.contains("nationals") => Ok(Self::WashingtonNationals),
            _ => Err(TeamDecodeError),
        }
    }
}

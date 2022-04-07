use serde::Serialize;
use strum;
use strum::EnumString;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Player {
    uuid: Uuid,
    name: String,
}

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum RankingType {
    Break,
    Build,
    PlayTime,
    Vote,
}

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum RankingAggregationTimeRange {
    All,
    LastOneYear,
    LastOneMonth,
    LastOneWeek,
    LastOneDay,
}

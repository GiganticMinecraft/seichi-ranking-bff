use crate::models::{AggregatedPlayerAttribution, RankedAttributionRecord};
use chrono::{DateTime, Utc};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use uuid::Uuid;

pub(crate) struct Player {
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) last_quit: DateTime<Utc>,
}

impl Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Player", 3)?;
        s.serialize_field("uuid", &self.uuid)?;
        s.serialize_field("name", &self.name)?;

        // OpenAPI Specificationでのdate-timeがRFC3339形式を要求している
        s.serialize_field("last_quit", &self.last_quit.to_rfc3339())?;
        s.end()
    }
}

#[derive(Serialize)]
pub(crate) struct RankingRecord {
    pub(crate) rank_position: u32,
    pub(crate) value: u64,
}

#[derive(Serialize)]
pub(crate) struct PlayerRankingRecord {
    pub(crate) player: Player,
    pub(crate) record: RankingRecord,
}

pub(crate) fn ranked_record_to_presentation_player_ranking_record<
    Attribution: AggregatedPlayerAttribution,
>(
    ranked_record: &RankedAttributionRecord<Attribution>,
) -> PlayerRankingRecord {
    let player = ranked_record.attribution_record.player.clone();

    PlayerRankingRecord {
        player: Player {
            uuid: player.uuid,
            name: player.name,
            last_quit: player.last_quit,
        },
        record: ranked_record_to_presentation_ranking_record(ranked_record),
    }
}

pub(crate) fn ranked_record_to_presentation_ranking_record<
    Attribution: AggregatedPlayerAttribution,
>(
    ranked_record: &RankedAttributionRecord<Attribution>,
) -> RankingRecord {
    RankingRecord {
        rank_position: ranked_record.rank,
        value: ranked_record.attribution_record.attribution.raw_u64_data(),
    }
}

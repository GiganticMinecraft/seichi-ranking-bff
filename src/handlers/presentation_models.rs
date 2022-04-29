use crate::models::{AggregatedPlayerAttribution, RankedAttributionRecord};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub(crate) struct Player {
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    // OpenAPI Specificationでの `format: date-time` によりRFC3339形式が要求されているが、
    // chrono 0.4.19 の `impl<Tz: TimeZone> serde::Serialize for DateTime<Tz>` は
    // RFC3339でシリアライズするようになっている。
    pub(crate) last_quit: DateTime<Utc>,
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

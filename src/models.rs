use async_trait::async_trait;
use serde::Serialize;
use std::iter;
use strum;
use strum::{EnumIter, EnumString};
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct Player {
    uuid: Uuid,
    name: String,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct BreakCount(u64);

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct BuildCount(u64);

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct PlayTicks(u64);

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct VoteCount(u64);

pub trait AggregatedPlayerAttribution: Ord + Clone {}

impl AggregatedPlayerAttribution for BreakCount {}
impl AggregatedPlayerAttribution for BuildCount {}
impl AggregatedPlayerAttribution for PlayTicks {}
impl AggregatedPlayerAttribution for VoteCount {}

#[derive(Clone)]
pub struct AttributionRecord<Attribution: AggregatedPlayerAttribution> {
    pub player: Player,
    pub attribution: Attribution,
}

#[derive(Clone)]
pub struct RankedAttributionRecord<Attribution: AggregatedPlayerAttribution> {
    pub rank: u32,
    pub attribution_record: AttributionRecord<Attribution>,
}

pub struct Ranking<Attribution: AggregatedPlayerAttribution> {
    /// 不変条件: `sorted_ranked_records` は「順位」が与えられており、次の条件を常に満たす。
    ///  - 任意の添え字 `i` について、
    ///    非負整数 `r ≤ i` が存在し、
    ///    すべての `r ≤ j ≤ i` について、
    ///    `sorted_ranked_records[j].rank.0 == r + 1`
    sorted_ranked_records: Vec<RankedAttributionRecord<Attribution>>,
}

pub struct RankingSlice<Attribution: AggregatedPlayerAttribution>(
    pub Vec<RankedAttributionRecord<Attribution>>,
);

impl<Attribution: AggregatedPlayerAttribution + Clone> Default for Ranking<Attribution> {
    fn default() -> Self {
        Ranking {
            sorted_ranked_records: vec![],
        }
    }
}

impl<Attribution: AggregatedPlayerAttribution + Clone> Ranking<Attribution> {
    pub fn hydrate_record_set(&mut self, records: Vec<AttributionRecord<Attribution>>) {
        struct ScanState<Attribution> {
            next_item_index: usize,
            previous_attribution: Attribution,
            previous_item_rank: u32,
        }

        let mut records = records;
        records.sort_by_key(|ar| ar.attribution.clone());
        records.reverse();

        let (first_record, tail_records) = match records.as_slice() {
            [first, tail @ ..] => (first, tail),
            [] => {
                self.sorted_ranked_records = vec![];
                return;
            }
        };

        let first_ranked_record = RankedAttributionRecord {
            rank: 1,
            attribution_record: first_record.clone(),
        };

        let initial_scan_state = ScanState {
            next_item_index: 0,
            previous_attribution: first_record.attribution.clone(),
            previous_item_rank: 1,
        };

        let ranked_tail_records = tail_records.iter().scan(initial_scan_state, |st, record| {
            let next_rank = if st.previous_attribution == record.attribution {
                st.previous_item_rank
            } else {
                assert!(st.previous_attribution < record.attribution);
                (st.next_item_index as u32) + 1
            };

            let next_ranked_record = RankedAttributionRecord {
                rank: next_rank,
                attribution_record: record.clone(),
            };

            st.next_item_index += 1;
            st.previous_attribution = record.attribution.clone();
            st.previous_item_rank = next_rank;

            Some(next_ranked_record)
        });

        self.sorted_ranked_records = iter::once(first_ranked_record)
            .chain(ranked_tail_records)
            .collect()
    }

    pub fn paginate(&self, offset: usize, limit: usize) -> RankingSlice<Attribution> {
        RankingSlice(self.sorted_ranked_records[offset..(offset + limit)].to_vec())
    }

    pub fn record_with_uuid(&self, uuid: Uuid) -> Option<RankedAttributionRecord<Attribution>> {
        self.sorted_ranked_records
            .iter()
            .find(|r| r.attribution_record.player.uuid == uuid)
            .cloned()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum AggregationTimeRange {
    #[strum(serialize = "all")]
    All,
    #[strum(serialize = "year")]
    LastOneYear,
    #[strum(serialize = "month")]
    LastOneMonth,
    #[strum(serialize = "week")]
    LastOneWeek,
    #[strum(serialize = "day")]
    LastOneDay,
}

#[async_trait]
pub trait AttributionRecordProvider<Attribution: AggregatedPlayerAttribution> {
    async fn get_all_attribution_records(
        &self,
        time_range: AggregationTimeRange,
    ) -> Vec<AttributionRecord<Attribution>>;
}

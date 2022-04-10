use crate::models::{
    AggregatedPlayerAttribution, AggregationTimeRange, AttributionRecordProvider, BreakCount,
    BuildCount, PlayTicks, Ranking, VoteCount,
};
use std::borrow::Borrow;
use std::sync::RwLock;

pub struct LockedRankingsForTimeRanges<Attribution: AggregatedPlayerAttribution> {
    all: RwLock<Ranking<Attribution>>,
    last_one_year: RwLock<Ranking<Attribution>>,
    last_one_month: RwLock<Ranking<Attribution>>,
    last_one_week: RwLock<Ranking<Attribution>>,
    last_one_day: RwLock<Ranking<Attribution>>,
}

/// We are manually defining this because derived impl requires Default on Attribution
impl<Attribution: AggregatedPlayerAttribution> Default
    for LockedRankingsForTimeRanges<Attribution>
{
    fn default() -> Self {
        LockedRankingsForTimeRanges {
            all: Default::default(),
            last_one_year: Default::default(),
            last_one_month: Default::default(),
            last_one_week: Default::default(),
            last_one_day: Default::default(),
        }
    }
}

impl<Attribution: AggregatedPlayerAttribution> LockedRankingsForTimeRanges<Attribution> {
    pub fn for_time_range(
        &self,
        time_range: AggregationTimeRange,
    ) -> &RwLock<Ranking<Attribution>> {
        match time_range {
            AggregationTimeRange::All => self.all.borrow(),
            AggregationTimeRange::LastOneYear => self.last_one_year.borrow(),
            AggregationTimeRange::LastOneMonth => self.last_one_month.borrow(),
            AggregationTimeRange::LastOneWeek => self.last_one_week.borrow(),
            AggregationTimeRange::LastOneDay => self.last_one_day.borrow(),
        }
    }
}

#[derive(Default)]
pub struct AppState {
    pub break_count_rankings: LockedRankingsForTimeRanges<BreakCount>,
    pub build_count_rankings: LockedRankingsForTimeRanges<BuildCount>,
    pub play_ticks_rankings: LockedRankingsForTimeRanges<PlayTicks>,
    pub vote_count_rankings: LockedRankingsForTimeRanges<VoteCount>,
}

pub struct AllAttributionRecordProviders {
    pub break_count: Box<dyn AttributionRecordProvider<BreakCount>>,
    pub build_count_provider: Box<dyn AttributionRecordProvider<BreakCount>>,
    pub play_ticks_provider: Box<dyn AttributionRecordProvider<BreakCount>>,
    pub vote_count_provider: Box<dyn AttributionRecordProvider<BreakCount>>,
}

pub async fn rehydration_process(state_ref: &AppState, providers: AllAttributionRecordProviders) {
    todo!("rehydrate state_ref with providers")
}

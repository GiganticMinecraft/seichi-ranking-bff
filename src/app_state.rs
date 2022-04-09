use crate::model::{
    AggregatedPlayerAttribution, AggregationTimeRange, BreakCount, BuildCount, PlayTicks, Ranking,
    VoteCount,
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

pub struct AppState<'a> {
    pub break_count_rankings: &'a LockedRankingsForTimeRanges<BreakCount>,
    pub build_count_rankings: &'a LockedRankingsForTimeRanges<BuildCount>,
    pub play_ticks_rankings: &'a LockedRankingsForTimeRanges<PlayTicks>,
    pub vote_count_rankings: &'a LockedRankingsForTimeRanges<VoteCount>,
}

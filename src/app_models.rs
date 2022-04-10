use crate::models::{
    AggregatedPlayerAttribution, AggregationTimeRange, AttributionRecordProvider, BreakCount,
    BuildCount, PlayTicks, Ranking, VoteCount,
};
use async_lock::RwLock;
use std::borrow::Borrow;
use std::ops::Deref;
use std::thread::sleep;
use strum::IntoEnumIterator;

pub struct LockedRankingsForTimeRanges<Attribution: AggregatedPlayerAttribution> {
    all: RwLock<Ranking<Attribution>>,
    last_one_year: RwLock<Ranking<Attribution>>,
    last_one_month: RwLock<Ranking<Attribution>>,
    last_one_week: RwLock<Ranking<Attribution>>,
    last_one_day: RwLock<Ranking<Attribution>>,
}

/// `derive` すると `Attribution` に `Default` 制約が付いてしまうので、手動でimplしている
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
    pub break_count_provider: Box<dyn AttributionRecordProvider<BreakCount> + Sync + Send>,
    pub build_count_provider: Box<dyn AttributionRecordProvider<BuildCount> + Sync + Send>,
    pub play_ticks_provider: Box<dyn AttributionRecordProvider<PlayTicks> + Sync + Send>,
    pub vote_count_provider: Box<dyn AttributionRecordProvider<VoteCount> + Sync + Send>,
}

async fn rehydrate<Attribution: AggregatedPlayerAttribution>(
    locked_rankings: &LockedRankingsForTimeRanges<Attribution>,
    provider: &(dyn AttributionRecordProvider<Attribution> + Sync + Send),
) {
    for time_range in AggregationTimeRange::iter() {
        let records = provider.get_all_attribution_records(time_range).await;
        let mut ranking = locked_rankings.for_time_range(time_range).write().await;
        ranking.hydrate_record_set(records);
    }
}

pub async fn rehydration_process(state_ref: &AppState, providers: AllAttributionRecordProviders) {
    const SLEEP_SECS: u64 = 120;

    loop {
        rehydrate(
            &state_ref.break_count_rankings,
            providers.break_count_provider.deref(),
        )
        .await;

        rehydrate(
            &state_ref.build_count_rankings,
            providers.build_count_provider.deref(),
        )
        .await;

        rehydrate(
            &state_ref.play_ticks_rankings,
            providers.play_ticks_provider.deref(),
        )
        .await;

        rehydrate(
            &state_ref.vote_count_rankings,
            providers.vote_count_provider.deref(),
        )
        .await;

        sleep(std::time::Duration::from_secs(SLEEP_SECS))
    }
}

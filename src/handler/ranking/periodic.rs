use std::any::Any;
use actix_web::{HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Local};
use qstring::QString;
use serde::{Serialize, Serializer};
use uuid::Uuid;

// FIXME bad name
// TODO enum-variant
trait RankingType {
    type ReturnProperty;
    // TODO: is this appropriate that belonging here?
    fn url_query_type(&self) -> &'static str;
}

struct RankingTypes;

impl RankingTypes {
    fn parse(raw: &str) -> Result<Box<dyn RankingType<ReturnProperty = dyn Any>>, RankingTypeCoercionError> {
        // FIXME: solve this puzzle
        if raw == Break.url_query_type() {
            Ok(Box::new(Break))
        } else if raw == Build.url_query_type() {
            Ok(Box::new(Build))
        } else if raw == PlayTime.url_query_type() {
            Ok(Box::new(PlayTime))
        } else if raw == Vote.url_query_type() {
            Ok(Box::new(Vote))
        } else {
            Err(RankingTypeCoercionError::InvalidSpecifier)
        }
    }
}

#[derive(Serialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct BreakCount(i64);
struct Break;
impl RankingType for Break {
    type ReturnProperty = BreakCount;

    fn url_query_type(&self) -> &'static str {
        "break"
    }
}

#[derive(Serialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct BuildCount(i64);
struct Build;
impl RankingType for Build {
    type ReturnProperty = BuildCount;

    fn url_query_type(&self) -> &'static str {
        "build"
    }
}

#[derive(Serialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct PlayTick(u64);

impl PlayTick {
    /// 端数切り捨て
    fn hours(&self) -> u64 {
        self.0 / 20 / 60 / 60
    }

    /// 端数切り捨て, 0..=59
    fn minute(&self) -> u64 {
        // FIXME incorrect
        self.0 / 20 / 60
    }

    /// 端数切り捨て, 0..=59
    fn second(&self) -> u64 {
        // FIXME incorrect
        self.0 / 20
    }
}

struct PlayTime;
impl RankingType for PlayTime {
    type ReturnProperty = PlayTick;

    fn url_query_type(&self) -> &'static str {
        "play_time"
    }
}

#[derive(Serialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct VoteCount(u64);

struct Vote;
impl RankingType for Vote {
    type ReturnProperty = VoteCount;

    fn url_query_type(&self) -> &'static str {
        "vote"
    }
}

enum RankingTypeCoercionError {
    InvalidSpecifier
}

enum RankingPeriod {
    Total,
    Yearly,
    Monthly,
    Weekly,
    Daily,
}

enum RankingPeriodCoercionError {
    InvalidSpecifier
}

impl TryFrom<&str> for RankingPeriod {
    type Error = RankingPeriodCoercionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "total" => Ok(Self::Total),
            "yearly" => Ok(Self::Yearly),
            "monthly" => Ok(Self::Monthly),
            "weekly" => Ok(Self::Weekly),
            "daily" => Ok(Self::Daily),
            _ => Err(RankingPeriodCoercionError::InvalidSpecifier)
        }
    }
}

#[derive(Serialize)]
struct CompatRankingSlice<RT: RankingType> where RT::ReturnProperty: Serialize + Ord {
    rank_count: usize,
    ranks: PlayerRank<RT>,
    total_count: usize,
}

#[derive(Serialize)]
struct PlayerRank<RT: RankingType> where RT::ReturnProperty: Serialize + Ord {
    rank: usize,
    kind: RT,
    data: GenericPlayerData<RT::ReturnProperty>,
    #[serde(serialize_with = "as_iso8601")]
    player_last_quit: LastQuit,
    player: Player
}

fn as_iso8601<S: Serializer>(l: &LastQuit, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&l.0.to_rfc3339().as_str())
}

#[derive(Serialize)]
struct GenericPlayerData<P: Serialize> {
    data: P
}

#[derive(Serialize)]
struct LastQuit(DateTime<Local>);

#[derive(Serialize)]
struct Player {
    name: String,
    uuid: Uuid
}

#[allow(clippy::unused_async)]
pub async fn periodic(req: HttpRequest) -> impl Responder {
    let qs: QString =  req.query_string().into();

    let raw_kind = qs.get("type").unwrap_or("break");
    let kind = match RankingTypes::parse(raw_kind) {
        Ok(t) => t,
        Err(_e) => return HttpResponse::BadRequest().body("")
    };

    let _duration: RankingPeriod = match qs.get("duration").unwrap_or("total").try_into() {
        Ok(t) => t,
        Err(_e) => return HttpResponse::BadRequest().body("")
    };

    HttpResponse::Ok().json(vec![
        1 // TODO
    ])
}
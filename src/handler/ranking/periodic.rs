use actix_web::body::BoxBody;
use actix_web::http::header::IF_UNMODIFIED_SINCE;
use actix_web::{HttpRequest, HttpResponse, Responder};
use qstring::QString;
use std::ops::Deref;
use std::str::FromStr;
use strum;
use strum::EnumString;

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
enum RankingType {
    Break,
    Build,
    PlayTime,
    Vote,
}

#[allow(dead_code)]
enum RankingTypeCoercionError {
    InvalidSpecifier,
}

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
enum RankingPeriod {
    Total,
    Yearly,
    Monthly,
    Weekly,
    Daily,
}

#[allow(dead_code)]
enum RankingPeriodCoercionError {
    InvalidSpecifier,
}

#[allow(clippy::unused_async)]
#[allow(clippy::future_not_send)]
#[actix_web::get("/seichi/ranking/v1/global/periodic")]
pub async fn periodic(req: HttpRequest) -> impl Responder {
    periodic_impl(&req).unwrap_or_else(|_| HttpResponse::BadRequest().body(""))
}

fn periodic_impl(req: &HttpRequest) -> anyhow::Result<HttpResponse<BoxBody>> {
    let qs: QString = req.query_string().into();
    let _kind = RankingType::from_str(qs.get("type").unwrap_or("break"))?;
    let _duration = RankingPeriod::from_str(qs.get("duration").unwrap_or("total"))?;
    Ok(req.headers().deref().get(IF_UNMODIFIED_SINCE).map_or_else(
        || HttpResponse::Ok().json(vec![1]),
        |_| HttpResponse::NotModified().body(""),
    ))
}

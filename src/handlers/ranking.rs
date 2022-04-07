use crate::model::{RankingPeriod, RankingType};
use actix_web::body::BoxBody;
use actix_web::http::header::IF_UNMODIFIED_SINCE;
use actix_web::web::Path;
use actix_web::{HttpRequest, HttpResponse, Responder};
use qstring::QString;
use serde_json::json;
use std::ops::Deref;
use std::str::FromStr;
use uuid::Uuid;

#[allow(dead_code)]
enum RankingTypeCoercionError {
    InvalidSpecifier,
}

#[allow(dead_code)]
enum RankingPeriodCoercionError {
    InvalidSpecifier,
}

#[allow(clippy::unused_async)]
#[allow(clippy::future_not_send)]
#[actix_web::get("/api/v1/ranking")]
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

#[allow(clippy::unused_async)]
#[allow(clippy::future_not_send)]
#[actix_web::get("/api/v1/player-ranks/{uuid}")]
pub async fn global_ranking_for_player(_req: HttpRequest, path: Path<Uuid>) -> impl Responder {
    let player_uuid = path.into_inner();
    // TODO: actual search

    HttpResponse::Ok().json(json!({
        "name": "$THIS_IS_DUMMY",
        "uuid": player_uuid
    }))
}

use crate::app_models::AppState;
use crate::models::AggregationTimeRange;
use actix_web::body::BoxBody;
use actix_web::web::Path;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use qstring::QString;
use std::str::FromStr;
use uuid::Uuid;

fn duration_not_recognized_response(duration_str: &str) -> HttpResponse<BoxBody> {
    HttpResponse::BadRequest().body(format!(
        "{duration_str} is not a recognized duration specifier."
    ))
}

fn time_range_from_qs(qs: &QString) -> &str {
    qs.get("time_range").unwrap_or("all")
}

#[allow(clippy::unused_async)]
#[allow(clippy::future_not_send)]
#[actix_web::get("/api/v1/ranking")]
pub async fn ranking(req: HttpRequest, _data: web::Data<&'static AppState>) -> impl Responder {
    let qs: QString = req.query_string().into();

    let time_range_specifier = time_range_from_qs(&qs);
    let _time_range = match AggregationTimeRange::from_str(time_range_specifier) {
        Ok(r) => r,
        Err(_) => return duration_not_recognized_response(time_range_specifier),
    };

    let _attribution_kind = qs.get("type").unwrap_or("break");

    todo!("fetch ranking from data and return appropriately paginated slice")
}

#[allow(clippy::unused_async)]
#[allow(clippy::future_not_send)]
#[actix_web::get("/api/v1/player-ranks/{uuid}")]
pub async fn player_rank(
    req: HttpRequest,
    path: Path<Uuid>,
    _data: web::Data<&'static AppState>,
) -> impl Responder {
    let qs: QString = req.query_string().into();

    let time_range_specifier = time_range_from_qs(&qs);
    let _time_range = match AggregationTimeRange::from_str(time_range_specifier) {
        Ok(r) => r,
        Err(_) => return duration_not_recognized_response(time_range_specifier),
    };

    let _attribution_kind = qs.get("type").unwrap_or("break");

    let _player_uuid = path.into_inner();

    todo!("fetch ranking from data and return information of the player")
}

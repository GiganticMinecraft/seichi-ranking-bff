use crate::app_models::AppState;
use crate::models::AggregationTimeRange;
use actix_web::body::BoxBody;
use actix_web::web::Path;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use qstring::QString;
use uuid::Uuid;

fn duration_not_recognized_response(duration_str: &str) -> HttpResponse<BoxBody> {
    HttpResponse::BadRequest().body(format!(
        "{duration_str} is not a recognized duration specifier."
    ))
}

fn time_range_from_qs(qs: &QString) -> &str {
    qs.get("time_range").unwrap_or("all")
}

fn parse_time_range(time_range_specifier: &str) -> Option<AggregationTimeRange> {
    match time_range_specifier {
        "all" => Some(AggregationTimeRange::All),
        "year" => Some(AggregationTimeRange::LastOneYear),
        "month" => Some(AggregationTimeRange::LastOneMonth),
        "week" => Some(AggregationTimeRange::LastOneWeek),
        "day" => Some(AggregationTimeRange::LastOneDay),
        _ => None,
    }
}

#[allow(clippy::unused_async)]
#[allow(clippy::future_not_send)]
#[actix_web::get("/api/v1/ranking")]
pub async fn ranking(req: HttpRequest, data: web::Data<&'static AppState>) -> impl Responder {
    let qs: QString = req.query_string().into();

    let time_range_specifier = time_range_from_qs(&qs);
    let time_range = match parse_time_range(time_range_specifier) {
        Some(r) => r,
        None => return duration_not_recognized_response(time_range_specifier),
    };

    let attribution_kind = qs.get("type").unwrap_or("break");

    todo!("fetch ranking from data and return appropriately paginated slice")
}

#[allow(clippy::unused_async)]
#[allow(clippy::future_not_send)]
#[actix_web::get("/api/v1/player-ranks/{uuid}")]
pub async fn player_rank(
    req: HttpRequest,
    path: Path<Uuid>,
    data: web::Data<&'static AppState>,
) -> impl Responder {
    let player_uuid = path.into_inner();

    let qs: QString = req.query_string().into();

    let time_range_specifier = time_range_from_qs(&qs);
    let time_range = match parse_time_range(time_range_specifier) {
        Some(r) => r,
        None => return duration_not_recognized_response(time_range_specifier),
    };

    let attribution_kind = qs.get("type").unwrap_or("break");

    todo!("fetch ranking from data and return information of the player")
}

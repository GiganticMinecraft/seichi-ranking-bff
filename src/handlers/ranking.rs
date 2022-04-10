use crate::app_models::AppState;
use crate::models::AggregationTimeRange;
use actix_web::web::Path;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use qstring::QString;
use uuid::Uuid;

#[allow(clippy::unused_async)]
#[allow(clippy::future_not_send)]
#[actix_web::get("/api/v1/ranking")]
pub async fn ranking(req: HttpRequest, data: web::Data<&'static AppState>) -> impl Responder {
    let qs: QString = req.query_string().into();

    let duration_specifier = qs.get("duration").unwrap_or("all");
    let duration = match duration_specifier {
        "all" => AggregationTimeRange::All,
        "year" => AggregationTimeRange::LastOneYear,
        "month" => AggregationTimeRange::LastOneMonth,
        "week" => AggregationTimeRange::LastOneWeek,
        "day" => AggregationTimeRange::LastOneDay,
        duration_specifier => {
            return HttpResponse::BadRequest().body(format!(
                "{duration_specifier} is not a recognized duration specifier."
            ))
        }
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
    let qs: QString = req.query_string().into();

    let player_uuid = path.into_inner();
    let attribution_kind = qs.get("type").unwrap_or("break");

    todo!("fetch ranking from data and return information of the player")
}

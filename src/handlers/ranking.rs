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

fn unknown_attribution_kind(attribution_kind: &str) -> HttpResponse<BoxBody> {
    HttpResponse::BadRequest().body(format!(
        "{attribution_kind} is not a recognized attribution specifier"
    ))
}

fn time_range_from_qs(qs: &QString) -> &str {
    qs.get("time_range").unwrap_or("all")
}

fn parse_usize_param(qs: &QString, param_name: &str) -> Option<usize> {
    qs.get(param_name)
        .and_then(|offset_str| offset_str.parse::<usize>().ok())
}

const RANKING_MAX_LIMIT_PER_REQUEST: usize = 1000;

#[allow(clippy::future_not_send)]
#[actix_web::get("/api/v1/ranking")]
pub async fn ranking(req: HttpRequest, data: web::Data<&'static AppState>) -> impl Responder {
    let qs: QString = req.query_string().into();

    let time_range_specifier = time_range_from_qs(&qs);
    let time_range = match AggregationTimeRange::from_str(time_range_specifier) {
        Ok(r) => r,
        Err(_) => return duration_not_recognized_response(time_range_specifier),
    };

    let limit = parse_usize_param(&qs, "limit").unwrap_or(20);
    let offset = parse_usize_param(&qs, "offset").unwrap_or(0);

    if limit > RANKING_MAX_LIMIT_PER_REQUEST {
        return HttpResponse::BadRequest().body(format!("{limit} is too large for a limit"));
    }

    let attribution_kind = qs.get("type").unwrap_or("break");

    match attribution_kind {
        "break" => {
            let _paginated_ranking = data
                .break_count_rankings
                .for_time_range(time_range)
                .read()
                .await
                .paginate(offset, limit);

            todo!()
        }
        "build" => {
            let _paginated_ranking = data
                .build_count_rankings
                .for_time_range(time_range)
                .read()
                .await
                .paginate(offset, limit);

            todo!()
        }
        "play_ticks" => {
            let _paginated_ranking = data
                .play_ticks_rankings
                .for_time_range(time_range)
                .read()
                .await
                .paginate(offset, limit);

            todo!()
        }
        "vote_count" => {
            let _paginated_ranking = data
                .vote_count_rankings
                .for_time_range(time_range)
                .read()
                .await
                .paginate(offset, limit);

            todo!()
        }
        other => unknown_attribution_kind(other),
    }
}

#[allow(clippy::future_not_send)]
#[actix_web::get("/api/v1/player-ranks/{uuid}")]
pub async fn player_rank(
    req: HttpRequest,
    path: Path<Uuid>,
    data: web::Data<&'static AppState>,
) -> impl Responder {
    let qs: QString = req.query_string().into();

    let time_range_specifier = time_range_from_qs(&qs);
    let time_range = match AggregationTimeRange::from_str(time_range_specifier) {
        Ok(r) => r,
        Err(_) => return duration_not_recognized_response(time_range_specifier),
    };

    let attribution_kind = qs.get("type").unwrap_or("break");

    let player_uuid = path.into_inner();

    match attribution_kind {
        "break" => {
            let _record = data
                .break_count_rankings
                .for_time_range(time_range)
                .read()
                .await
                .record_with_uuid(player_uuid);

            todo!()
        }
        "build" => {
            let _record = data
                .build_count_rankings
                .for_time_range(time_range)
                .read()
                .await
                .record_with_uuid(player_uuid);

            todo!()
        }
        "play_ticks" => {
            let _record = data
                .play_ticks_rankings
                .for_time_range(time_range)
                .read()
                .await
                .record_with_uuid(player_uuid);

            todo!()
        }
        "vote_count" => {
            let _record = data
                .vote_count_rankings
                .for_time_range(time_range)
                .read()
                .await
                .record_with_uuid(player_uuid);

            todo!()
        }
        other => unknown_attribution_kind(other),
    }
}

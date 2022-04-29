use crate::app_models::AppState;
use crate::handlers::presentation_models::{
    ranked_record_to_presentation_player_ranking_record,
    ranked_record_to_presentation_ranking_record,
};
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
#[actix_web::get("/ranking")]
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

    macro_rules! respond_using {
        ($ranking:expr) => {{
            let paginated_ranking = $ranking
                .for_time_range(time_range)
                .read()
                .await
                .paginate(offset, limit);

            HttpResponse::Ok().json(
                paginated_ranking
                    .0
                    .into_iter()
                    .map(|r| ranked_record_to_presentation_player_ranking_record(&r))
                    .collect::<Vec<_>>(),
            )
        }};
    }

    match attribution_kind {
        "break" => respond_using!(data.break_count_rankings),
        "build" => respond_using!(data.build_count_rankings),
        "play_ticks" => respond_using!(data.play_ticks_rankings),
        "vote_count" => respond_using!(data.vote_count_rankings),
        other => unknown_attribution_kind(other),
    }
}

fn record_with_uuid_not_found(
    attribution_kind: &str,
    time_range: AggregationTimeRange,
    uuid: Uuid,
) -> HttpResponse<BoxBody> {
    HttpResponse::NotFound().body(format!(
        "record with {uuid} for kind={attribution_kind}, time-range={time_range} not found"
    ))
}

#[allow(clippy::future_not_send)]
#[actix_web::get("/player-ranks/{uuid}")]
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

    macro_rules! respond_using {
        ($ranking:expr) => {{
            let record_option = $ranking
                .for_time_range(time_range)
                .read()
                .await
                .record_with_uuid(player_uuid);

            let record = match record_option {
                Some(r) => r,
                None => {
                    return record_with_uuid_not_found(attribution_kind, time_range, player_uuid)
                }
            };

            HttpResponse::Ok().json(ranked_record_to_presentation_ranking_record(&record))
        }};
    }

    match attribution_kind {
        "break" => respond_using!(data.break_count_rankings),
        "build" => respond_using!(data.build_count_rankings),
        "play_ticks" => respond_using!(data.play_ticks_rankings),
        "vote_count" => respond_using!(data.vote_count_rankings),
        other => unknown_attribution_kind(other),
    }
}

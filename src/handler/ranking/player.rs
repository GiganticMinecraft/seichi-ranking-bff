use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::web::Path;
use serde_json::json;
use uuid::Uuid;

#[allow(clippy::unused_async)]
pub async fn global_ranking_for_player(req: HttpRequest, path: Path<Uuid>) -> impl Responder {
    let player_uuid = path.into_inner();
    // TODO: actual search

    HttpResponse::Ok().json(json!({
        "name": "$THIS_IS_DUMMY",
        "uuid": player_uuid
    }))
}

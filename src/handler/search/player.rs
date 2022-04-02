use crate::model::Player;
use actix_web::{HttpRequest, HttpResponse, Responder};
use qstring::QString;
use serde::Serialize;

#[derive(Serialize)]
struct Model {
    result_count: usize,
    found_players: Vec<Player>,
}

#[allow(clippy::unused_async)]
#[actix_web::get("/seichi/search/v1/player")]
pub async fn search(req: HttpRequest) -> impl Responder {
    let qs = QString::from(req.query_string());
    let _query = match qs.get("q") {
        None => {
            return HttpResponse::BadRequest().body("no query given");
        }
        Some(a) => a,
    };

    let _limit = match qs.get("lim") {
        None => 20,
        Some(a) => match a.parse() {
            Ok(a) => a,
            Err(v) => return HttpResponse::BadRequest().body(format!("{}", v)),
        },
    };

    // TODO: actual search
    let found_players = vec![];
    HttpResponse::Ok().json(create_response(found_players))
}

fn create_response(found_players: Vec<Player>) -> Model {
    Model {
        result_count: found_players.len(),
        found_players,
    }
}

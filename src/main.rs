#![deny(clippy::all, clippy::cargo)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cargo_common_metadata)]

mod config;
mod handler;
mod model;

use crate::config::{Config, FromEnv};
use crate::handler::ranking::player::global_ranking_for_player;
use crate::handler::search::player::search;
use actix_web::error::JsonPayloadError;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer};
use anyhow::{Context, Result};
use log::{error, info, trace, warn};

struct Initialization;

impl Initialization {
    fn setup_logger() -> Result<(), fern::InitError> {
        use fern::colors::ColoredLevelConfig;
        let colors = ColoredLevelConfig::new();

        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    colors.color(record.level()),
                    message
                ));
            })
            .level(log::LevelFilter::Trace)
            .chain(std::io::stdout())
            .chain(fern::log_file("output.log")?)
            .apply()?;
        Ok(())
    }
}

fn json_error_handler(
    err: actix_web::error::JsonPayloadError,
    req: &HttpRequest,
) -> actix_web::error::Error {
    use actix_web::error::InternalError;
    let detail = err.to_string();
    error!("error during handling JSON, in {:?}: {:?}", req, err);

    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    InternalError::from_response(err, resp).into()
}

#[actix_web::main]
async fn main() -> Result<()> {
    // This function contains code snippet which is licensed with Apache License 2.0
    // from https://github.com/actix/examples.
    // See https://www.apache.org/licenses/LICENSE-2.0.txt for full text.
    println!("starting");
    if let Err(err) = Initialization::setup_logger().context("failed to setup logger") {
        eprintln!("failed to initialize logger: {err:?}");
    }

    trace!("Reading config...");
    let config = Config::from_env()?;

    trace!("building HttpServer");
    let http_server = HttpServer::new(|| {
        use crate::handler::ranking::periodic::periodic;

        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(json_error_handler)
            .service(periodic)
            .service(global_ranking_for_player)
            .service(search)
    });
    trace!("binding ports");

    http_server
        .bind(format!("{}:{}", config.http_config.host, config.http_config.port.0))?
        .run()
        .await?;

    info!("stopped");
    Ok(())
}

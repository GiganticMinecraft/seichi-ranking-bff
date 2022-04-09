#![deny(clippy::all, clippy::cargo)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cargo_common_metadata)]

use actix_web::{App, HttpServer};
use anyhow::{Context, Result};
use log::{info, trace, warn};
use seichi_ranking_bff::{
    config::{Config, FromEnv},
    handlers::{ranking::player_rank, ranking::ranking},
};

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

#[actix_web::main]
async fn main() -> Result<()> {
    println!("starting");
    if let Err(err) = setup_logger().context("failed to setup logger") {
        eprintln!("failed to initialize logger: {err:?}");
    }

    trace!("Reading config...");
    let config = Config::from_env()?;

    trace!("building HttpServer");
    let http_server = HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(ranking)
            .service(player_rank)
    });

    trace!("binding ports");
    http_server
        .bind(format!(
            "{}:{}",
            config.http_config.host, config.http_config.port.0
        ))?
        .run()
        .await?;

    info!("stopped");
    Ok(())
}

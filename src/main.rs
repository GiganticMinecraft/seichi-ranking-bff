#![deny(clippy::all, clippy::cargo)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cargo_common_metadata)]

mod handler;
mod config;
mod model;
mod ext;

use std::fs::File;
use std::io::BufReader;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::error::JsonPayloadError;
use anyhow::{bail, Context};
use log::{error, info, trace, warn};
use once_cell::sync::OnceCell;
use rustls::{Certificate, PrivateKey, ServerConfig};
use crate::config::Config;
use crate::handler::ranking::player::global_ranking_for_player;
use crate::handler::search::player::search;
use crate::ext::buffered::BufferedRead;

static RUNNING_CONFIG: OnceCell<Config> = OnceCell::new();

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

    fn load_ssl_keys() -> anyhow::Result<(Vec<Certificate>, PrivateKey)> {
        use rustls_pemfile::{certs, pkcs8_private_keys};
        trace!("loading cert.pem");
        let cert_chain = {
            let cert_file = &mut File::open("cert.pem")?.buffered();
            certs(cert_file).unwrap().iter().map(|bytes| Certificate(bytes.clone())).collect()
        };
        trace!("loading key.pem");
        let mut keys = {
            let key_file = &mut File::open("key.pem")?.buffered();
            pkcs8_private_keys(key_file).unwrap().iter().map(|bytes| PrivateKey(bytes.clone())).collect::<Vec<_>>()
        };

        if keys.is_empty() {
            warn!("Could not locate PKCS 8 private keys");
            bail!("SSL keys were not available")
        }

        Ok((cert_chain, keys.remove(0)))
    }

    fn set_config() {
        trace!("Reading config...");
        let running_config = File::open("data/config.json").unwrap().buffered();
        RUNNING_CONFIG.set(serde_json::from_reader(BufReader::new(running_config)).unwrap()).expect("set failed");
    }
}

fn json_error_handler(err: actix_web::error::JsonPayloadError, req: &HttpRequest) -> actix_web::error::Error {
    use actix_web::error::InternalError;
    let detail = err.to_string();
    error!("error during handling JSON, in {:?}: {:?}", req, err);

    let resp = match &err {
        JsonPayloadError::ContentType => {
            HttpResponse::UnsupportedMediaType().body(detail)
        }
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    InternalError::from_response(err, resp).into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // This function contains code snippet which is licensed with Apache License 2.0
    // from https://github.com/actix/examples.
    // See https://www.apache.org/licenses/LICENSE-2.0.txt for full text.
    println!("starting");
    match Initialization::setup_logger().context("failed to setup logger") {
        Ok(_) => {}
        Err(err) => {
            eprintln!("failed to initialize logger: {:?}", err);
        }
    }

    // load SSL keys
    let session_config = {
        Initialization::load_ssl_keys().map(|(cert_chain, key_der)| {
            ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(cert_chain, key_der).unwrap()
        })
    };

    Initialization::set_config();
    trace!("building HttpServer");
    let http_server = HttpServer::new(|| {
        use crate::handler::ranking::{periodic::periodic};

        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(json_error_handler)
            .service(periodic)
            .service(global_ranking_for_player)
            .service(search)
    });
    trace!("binding ports");
    let http_server = match session_config {
        Ok(session_config) => {
            http_server
                .bind_rustls(format!("127.0.0.1:{}", RUNNING_CONFIG.get().unwrap().ports.https.0), session_config)?
        }
        Err(err) => {
            warn!("SSL connection is not enabled. Reason: {}", err);
            http_server
        }
    };

    http_server
        .bind(format!("127.0.0.1:{}", RUNNING_CONFIG.get().unwrap().ports.http.0))?
        .run()
        .await?;

    info!("stopped");
    Ok(())
}
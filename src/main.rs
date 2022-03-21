#![deny(clippy::all, clippy::cargo)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cargo_common_metadata)]

mod handler;
mod config;

use std::fs::File;
use std::io::BufReader;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::error::JsonPayloadError;
use anyhow::{Context};
use log::{error, info, trace};
use once_cell::sync::OnceCell;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use crate::config::config::Config;

static RUNNING_CONFIG: OnceCell<Config> = OnceCell::new();

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

fn load_ssl_keys() -> (Vec<Certificate>, PrivateKey) {
    trace!("loading cert.pem");
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap().iter().map(|a| Certificate(a.clone())).collect();
    trace!("loading key.pem");
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    let mut keys = pkcs8_private_keys(key_file).unwrap().iter().map(|x| PrivateKey(x.clone())).collect::<Vec<_>>();
    if keys.is_empty() {
        error!("Could not locate PKCS 8 private keys.");
    }

    (cert_chain, keys.remove(0))
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
    match setup_logger().context("failed to setup logger") {
        Ok(_) => {}
        Err(err) => {
            eprintln!("failed to initialize logger: {:?}", err);
        }
    }

    // load SSL keys
    let session_config = {
        let (cert_chain, key_der) = load_ssl_keys();
        ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der).unwrap()
    };

    trace!("Reading config...");
    let running_config = File::open("data/config.json").unwrap();
    RUNNING_CONFIG.set(serde_json::from_reader(BufReader::new(running_config)).unwrap()).expect("set failed");
    trace!("building HttpServer");
    let http_server = HttpServer::new(|| {
        use crate::handler::ranking::{periodic::periodic};

        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(json_error_handler)
            .service(
                web::resource("/ranking/periodic")
                    .route(
                        web::get()
                            .to(periodic) // periodic-ranking handler
                    )

            )
            /*
            .service(
                web::resource("/ranking/player/{}")
                    .route(
                        web::get()
                            .to(todo!()) // player-specific handler
                    )
            )
            .service(
                web::resource("/general/player/{}")
                    .route(
                        web::get()
                            .to(todo!())
                    )
            )

             */
    });
    trace!("binding ports");
    http_server
        .bind_rustls(format!("127.0.0.1:{}", RUNNING_CONFIG.get().unwrap().ports.https.0), session_config)?
        .bind(format!("127.0.0.1:{}", RUNNING_CONFIG.get().unwrap().ports.http.0))?
        .run()
        .await?;

    info!("stopped");
    Ok(())
}
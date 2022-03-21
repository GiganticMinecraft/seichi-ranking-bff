mod handler;
mod config;

use std::fs::File;
use std::io::BufReader;
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::web::JsonConfig;
use anyhow::Context;
use log::{error, info, trace};
use once_cell::sync::OnceCell;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use crate::config::config::Config;

static RUNNING_CONFIG: OnceCell<Config> = OnceCell::new();

fn setup_logger() -> Result<(), fern::InitError> {
    use fern::colors::*;
    let mut colors = ColoredLevelConfig::new();

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
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
    let mut session_config = {
        match load_ssl_keys() {
            (cert_chain, key_der) => {
                ServerConfig::builder()
                    .with_safe_defaults()
                    .with_no_client_auth()
                    .with_single_cert(cert_chain, key_der).unwrap()
            }
        }
    };

    trace!("Reading config...");
    let running_config = File::open("data/config.json").unwrap();
    RUNNING_CONFIG.set(serde_json::from_reader(BufReader::new(running_config)).unwrap());
    trace!("building HttpServer");
    let mut http_server = HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(
                JsonConfig::default().error_handler(handler::json_error_handler)
            )
            .service(
                web::resource("/ranking/periodic")
                    .route(
                        web::get()
                            .to(todo!()) // periodic-ranking handler
                    )

            )
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
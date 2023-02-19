use actix_web::{web, App, HttpServer, Responder};
use log::*;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::Deserialize;
use std::{sync::Mutex, time::Duration};

struct AppState {
    session_served: Mutex<u32>,
}

#[derive(Deserialize)]
struct Delay {
    delay: u32,
}

#[actix_web::get("/slow")]
async fn slow(param: web::Query<Delay>, state: web::Data<AppState>) -> impl Responder {
    let delay = param.delay;
    tokio::time::sleep(Duration::from_millis(delay.into())).await;

    info!("Served response after {delay} milliseconds");
    let mut count = state.session_served.lock().unwrap();
    *count += 1;

    format!("Received response after {delay} milliseconds")
}

#[actix_web::get("/served")]
async fn served(state: web::Data<AppState>) -> impl Responder {
    let served = state.session_served.lock().unwrap();
    format!("Served {served} slow requests this session")
}

#[actix_web::main]
async fn main() -> ::std::io::Result<()> {
    env_logger::init();

    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    ssl_builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    ssl_builder.set_certificate_chain_file("cert.pem").unwrap();

    let state = web::Data::new(AppState {
        session_served: Mutex::new(0u32),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::scope("/v1").service(slow).service(served))
    })
    .bind_openssl("127.0.0.1:8080", ssl_builder)?
    .run()
    .await?;

    Ok(())
}

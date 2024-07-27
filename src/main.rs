

use std::sync::{Arc, Mutex};

use actix_web::{
    get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder, Result,
};
// use tokio::sync::Mutex;
use tokio_postgres::NoTls;
mod models;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }
    env_logger::init();
    dotenv::dotenv().ok();

    let address = (*utils::constants::ADDRESS).clone();
    let port = (*utils::constants::PORT).clone();
    let db_url = (*utils::constants::DATABASE_URL).clone();

    let (client, connection) = tokio_postgres::connect(&db_url, NoTls)
        .await
        .expect("Failed to connect to the database");
    let client: Arc<Mutex<tokio_postgres::Client>> = Arc::new(Mutex::new(client));
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(client.clone()))
            .configure(routes::home_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}

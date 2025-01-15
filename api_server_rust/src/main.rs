use actix_web::{middleware::Logger, web, web::Data, App, HttpServer};
use log::info;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub mod helpers;
pub mod models;
pub mod responses;
pub mod routes;
pub mod token;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub access_token_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::new().parse_filters("info").init();

    dotenvy::dotenv().expect("Issue loading dotenv");

    let database_url = env::var("DATABASE_URL").expect("Issue finding the db url from env files");
    let access_token_secret =
        env::var("ACCESS_SECRET").expect("Issue finding the access token secret from env files");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Issue connecting to the database");

    info!("Starting Actix Web server...");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(AppState {
                db: pool.clone(),
                access_token_secret: access_token_secret.clone(),
            }))
            .service(
                web::scope("/api/v1/user")
                    .route(
                        "/create",
                        web::post().to(routes::user::create_user::create_user),
                    )
                    .route(
                        "/login",
                        web::post().to(routes::user::login_user::login_user),
                    ),
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

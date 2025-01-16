use actix_web::{
    middleware::{from_fn, Logger},
    web::{self, Data},
    App, HttpServer,
};
use log::info;
use middleware::auth_middleware;
use redis::Client;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, sync::Arc};
use tokio::sync::Mutex;

pub mod helpers;
pub mod middleware;
pub mod models;
pub mod responses;
pub mod routes;
pub mod token;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub ts1: Pool<Postgres>,
    pub ts2: Pool<Postgres>,
    pub access_token_secret: String,
    shared_state: Arc<Mutex<SharedState>>,
    redis_client: Client,
}

struct SharedState {
    counter: u32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::new().parse_filters("info").init();

    dotenvy::dotenv().expect("Issue loading dotenv");

    let database_url = env::var("DATABASE_URL").expect("Issue finding the db url from env files");
    let access_token_secret =
        env::var("ACCESS_SECRET").expect("Issue finding the access token secret from env files");
    let ts1_url = env::var("DATABASE_URL_TS1").expect("Issue finding the db url for ts1");
    let ts2_url = env::var("DATABASE_URL_TS2").expect("Issue finding the db url for ts2");

    let redis_client =
        redis::Client::open("redis://127.0.0.1/").expect("Issue connecting to redis");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Issue connecting to the database");

    let ts1 = PgPoolOptions::new()
        .max_connections(3)
        .connect(&ts1_url)
        .await
        .expect("Issue connecting to the database");

    let ts2 = PgPoolOptions::new()
        .max_connections(2)
        .connect(&ts2_url)
        .await
        .expect("Issue connecting to the database");

    info!("Starting Actix Web server...");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(AppState {
                db: pool.clone(),
                ts1: ts1.clone(),
                ts2: ts2.clone(),
                access_token_secret: access_token_secret.clone(),
                shared_state: Arc::new(Mutex::new(SharedState { counter: 1 })),
                redis_client: redis_client.clone(),
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
                    )
                    .service(
                        web::scope("/protected")
                            .wrap(from_fn(auth_middleware::auth_middleware))
                            .route(
                                "/currentUser",
                                web::get().to(routes::user::current_user::get_current_user),
                            ),
                    ),
            )
            .service(
                web::scope("/api/v1/url").service(
                    web::scope("/protected")
                        .wrap(from_fn(auth_middleware::auth_middleware))
                        .route("/create", web::post().to(routes::urls::create::create_url)),
                ),
            )
            .route("/{hash}", web::get().to(routes::urls::get_url::get_url))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

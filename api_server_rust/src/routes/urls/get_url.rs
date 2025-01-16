use actix_web::{http::header, web, HttpResponse, Responder};
use redis::AsyncCommands;

use crate::{models::url::UrlFromDB, responses::error::GeneralError, AppState};

pub async fn get_url(hash: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let hash = hash.into_inner();

    let data_from_redis = get_data_redis(&hash, &data.redis_client).await;
    if let Some(url) = data_from_redis {
        return HttpResponse::PermanentRedirect()
            .append_header((header::LOCATION, url))
            .finish();
    }

    let url_db_result = sqlx::query_as::<_, UrlFromDB>("select * from urls where hash=$1")
        .bind(&hash)
        .fetch_optional(&data.db)
        .await;

    if url_db_result.is_err() {
        return HttpResponse::InternalServerError().json(GeneralError {
            message: "Issue fetching the index".to_string(),
        });
    }

    if url_db_result.as_ref().unwrap().is_none() {
        return HttpResponse::NotFound().json(GeneralError {
            message: "Not found".to_string(),
        });
    }

    let original_url = url_db_result.unwrap().unwrap().original_url;

    set_data_redis(&hash, &original_url, &data.redis_client).await;

    HttpResponse::PermanentRedirect()
        .append_header((header::LOCATION, original_url))
        .finish()
}

async fn get_data_redis(hash: &str, client: &redis::Client) -> Option<String> {
    let con = client.get_multiplexed_async_connection().await;
    if con.is_err() {
        return None;
    }

    let res = con.unwrap().get(hash).await;
    if res.is_err() {
        return None;
    }
    res.unwrap()
}

async fn set_data_redis(key: &str, value: &str, client: &redis::Client) -> Option<()> {
    let con = client.get_multiplexed_async_connection().await;
    if con.is_err() {
        return None;
    }

    let ttl = 3600;
    let _: () = con.unwrap().set_ex(key, value, ttl).await.unwrap();
    Some(())
}

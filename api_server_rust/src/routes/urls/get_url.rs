use actix_web::{http::header, web, HttpResponse, Responder};

use crate::{models::url::UrlFromDB, responses::error::GeneralError, AppState};

pub async fn get_url(hash: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let hash = hash.into_inner();
    let url_db_result = sqlx::query_as::<_, UrlFromDB>("select * from urls where hash=$1")
        .bind(hash)
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

    HttpResponse::PermanentRedirect()
        .append_header((
            header::LOCATION,
            url_db_result.unwrap().unwrap().original_url,
        ))
        .finish()
}

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};

use crate::{
    middleware::auth_middleware::UserData,
    models::{
        ticket::RowTicket,
        url::{UrlFromDB, UrlReqBody},
    },
    responses::error::GeneralError,
    AppState,
};

pub async fn create_url(
    req: HttpRequest,
    data: web::Data<AppState>,
    new_url: web::Json<UrlReqBody>,
) -> impl Responder {
    if req.extensions().get::<UserData>().is_none() {
        return HttpResponse::InternalServerError().json(GeneralError {
            message: "Issue talking to the database".to_string(),
        });
    }
    let extensions = req.extensions();
    let user_data = extensions.get::<UserData>().unwrap();
    let mut counter_data = data.shared_state.lock().await;
    let server_number = counter_data.counter;
    counter_data.counter = (counter_data.counter + 1) % 6;

    let database_server = if server_number % 2 == 0 {
        data.ts1.clone()
    } else {
        data.ts2.clone()
    };

    let index_data = sqlx::query_as::<_, RowTicket>(
        "update ranges set current=current+1 returning current-1 as prev;",
    )
    .fetch_optional(&database_server)
    .await;

    if index_data.is_err() {
        return HttpResponse::InternalServerError().json(GeneralError {
            message: "Issue fetching the index".to_string(),
        });
    }

    if index_data.as_ref().unwrap().is_none() {
        return HttpResponse::InternalServerError().json(GeneralError {
            message: "Issue fetching the index".to_string(),
        });
    }

    let res = crate::helpers::hash_generator::character_mapper(index_data.unwrap().unwrap().prev);
    let url_db_insert_result = sqlx::query_as::<_, UrlFromDB>(
        "insert into urls(hash, original_url, creator_id) values ($1, $2, $3) returning *",
    )
    .bind(res)
    .bind(new_url.0.url)
    .bind(user_data.user_id)
    .fetch_optional(&data.db)
    .await;

    if url_db_insert_result.is_err() {
        return HttpResponse::InternalServerError().json(GeneralError {
            message: "Issue writing to the database".to_string(),
        });
    }

    if url_db_insert_result.as_ref().unwrap().is_none() {
        return HttpResponse::InternalServerError().json(GeneralError {
            message: "Issue writing to the database".to_string(),
        });
    }

    HttpResponse::Created().json(url_db_insert_result.unwrap().unwrap())
}

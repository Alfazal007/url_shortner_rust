use actix_web::{web, HttpResponse, Responder};

use crate::{
    models::{ticket::RowTicket, url::UrlReqBody},
    responses::error::GeneralError,
    AppState,
};

pub async fn create_url(
    data: web::Data<AppState>,
    new_url: web::Json<UrlReqBody>,
) -> impl Responder {
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

    if index_data.unwrap().is_none() {
        return HttpResponse::InternalServerError().json(GeneralError {
            message: "Issue fetching the index".to_string(),
        });
    }

    HttpResponse::InternalServerError().json(GeneralError {
        message: "".to_string(),
    })
}

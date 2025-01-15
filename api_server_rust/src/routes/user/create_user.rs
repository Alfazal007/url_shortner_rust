use crate::{
    models::user::{User, UserReqBody},
    responses::error::{GeneralError, ValidationErrorsToBeReturned},
    AppState,
};
use actix_web::{web, HttpResponse, Responder};
use validator::Validate;

pub async fn create_user(
    data: web::Data<AppState>,
    new_user: web::Json<UserReqBody>,
) -> impl Responder {
    if let Err(e) = new_user.validate() {
        let mut validation_errors: Vec<String> = Vec::new();
        for (_, err) in e.field_errors().iter() {
            if let Some(message) = &err[0].message {
                validation_errors.push(message.clone().into_owned());
            }
        }
        return HttpResponse::BadRequest().json(ValidationErrorsToBeReturned {
            errors: validation_errors,
        });
    }

    let existing_user = sqlx::query_as::<_, User>("select * from users where username=$1")
        .bind(new_user.0.username.clone())
        .fetch_optional(&data.db)
        .await;

    if existing_user.is_err() {
        return HttpResponse::InternalServerError().json(GeneralError {
            message: "Issue talking to the database".to_string(),
        });
    }

    if existing_user.unwrap().is_some() {
        return HttpResponse::BadRequest().json(GeneralError {
            message: "User with this username exists".to_string(),
        });
    }

    let hashed = bcrypt::hash(&new_user.0.password, 12);
    if hashed.is_err() {
        return HttpResponse::InternalServerError().json(GeneralError {
            message: "Issue hashing the password".to_string(),
        });
    }

    let user_created = sqlx::query_as::<_, User>(
        "insert into users(username, password) values ($1, $2) returning *",
    )
    .bind(new_user.0.username)
    .bind(hashed.unwrap())
    .fetch_optional(&data.db)
    .await;

    if user_created.is_err() {
        return HttpResponse::InternalServerError().json(GeneralError {
            message: "Issue talking to the database".to_string(),
        });
    }

    if user_created.as_ref().unwrap().is_none() {
        return HttpResponse::BadRequest().json(GeneralError {
            message: "Issue writing to the database".to_string(),
        });
    }
    HttpResponse::Ok().json(user_created.unwrap())
}

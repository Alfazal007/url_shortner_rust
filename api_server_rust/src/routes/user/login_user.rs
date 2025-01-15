use actix_web::{
    cookie::{Cookie, SameSite},
    web, HttpResponse, Responder,
};
use serde::Serialize;

use crate::{
    models::user::{UserReqBody, UserWithPassword},
    responses::error::{GeneralError, ValidationErrorsToBeReturned},
    AppState,
};
use validator::Validate;

#[derive(Serialize)]
struct LoginResponse {
    #[serde(rename = "accessToken")]
    access_token: String,
}

pub async fn login_user(
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

    let existing_user =
        sqlx::query_as::<_, UserWithPassword>("select * from users where username=$1")
            .bind(new_user.0.username.clone())
            .fetch_optional(&data.db)
            .await;

    if existing_user.is_err() {
        return HttpResponse::InternalServerError().json(GeneralError {
            message: "Issue talking to the database".to_string(),
        });
    }

    if existing_user.as_ref().unwrap().is_none() {
        return HttpResponse::NotFound().json(GeneralError {
            message: "User not found".to_string(),
        });
    }

    let valid = bcrypt::verify(
        new_user.0.password,
        &existing_user.as_ref().unwrap().as_ref().unwrap().password,
    );

    if valid.is_err() {
        return HttpResponse::BadRequest().json(GeneralError {
            message: "Issue talking to the database".to_string(),
        });
    }

    if !valid.unwrap() {
        return HttpResponse::BadRequest().json(GeneralError {
            message: "Wrong password".to_string(),
        });
    }
    let token = crate::token::generate_token::generate_token(
        &existing_user.as_ref().unwrap().as_ref().unwrap().username,
        existing_user.as_ref().unwrap().as_ref().unwrap().id,
        &data.access_token_secret,
    );

    if token.is_err() {
        return HttpResponse::InternalServerError().json(GeneralError {
            message: "Issue generating the token".to_string(),
        });
    }

    let cookie = Cookie::build("accessToken", token.as_ref().unwrap())
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None)
        .finish();

    HttpResponse::Ok().cookie(cookie).json(LoginResponse {
        access_token: token.unwrap(),
    })
}

use validator::Validate;

#[derive(sqlx::FromRow, serde::Deserialize, Debug, serde::Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(serde::Deserialize, Validate)]
pub struct UserReqBody {
    #[validate(length(
        min = 6,
        max = 20,
        message = "Username should be between 6 and 20 length"
    ))]
    pub username: String,
    #[validate(length(
        min = 6,
        max = 20,
        message = "Password should be between 6 and 20 length"
    ))]
    pub password: String,
}

#[derive(sqlx::FromRow, serde::Deserialize, Debug, serde::Serialize)]
pub struct UserWithPassword {
    pub id: i32,
    pub username: String,
    pub password: String,
}

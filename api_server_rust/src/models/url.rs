use validator::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct UrlReqBody {
    #[validate(url(message = "The provided URL is not valid."))]
    pub url: String,
}

#[derive(sqlx::FromRow, serde::Deserialize, Debug, serde::Serialize)]
pub struct UrlFromDB {
    pub hash: String,
    pub original_url: String,
}

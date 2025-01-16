use validator::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct UrlReqBody {
    #[validate(url(message = "The provided URL is not valid."))]
    pub url: String,
}

#[derive(serde::Serialize)]
pub struct GeneralError {
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct ValidationErrorsToBeReturned {
    pub errors: Vec<String>,
}

#[derive(sqlx::FromRow, serde::Deserialize, Debug, serde::Serialize)]
pub struct RowTicket {
    pub prev: i32,
}

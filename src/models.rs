#[derive(Queryable)]
pub struct Stock {
    pub id: i32,
    pub ticker: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

use super::schema::stocks;

#[derive(Queryable)]
pub struct Stock {
    pub id: i32,
    pub ticker: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[table_name = "stocks"]
pub struct NewStock<'a> {
    pub ticker: &'a str, // This has a max size of 4.
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostQueries {
    pub format: Option<String>,
}

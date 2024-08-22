use serde::Deserialize;

// TODO: 適切な定義ファイルに移動させる.
#[derive(Deserialize)]
pub struct PostQueries {
    pub format: Option<String>,
}

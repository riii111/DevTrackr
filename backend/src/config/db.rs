use dotenv::dotenv;
use mongodb::{Client, Database};
use std::error::Error;

pub async fn init_db() -> Result<Database, Box<dyn Error>> {
    dotenv().ok();
    let uri = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let client = Client::with_uri_str(uri).await?;
    let database = client.database("devtrackr_db");
    Ok(database)
}

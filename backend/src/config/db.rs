use dotenv::dotenv;
use mongodb::{Client, Database};
use std::error::Error;
use std::sync::Arc;

pub async fn init_db() -> Result<Arc<Database>, Box<dyn Error>> {
    dotenv().ok();
    let uri = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let client = Client::with_uri_str(uri).await?;
    let database = client.database("devtrackr_db");
    Ok(Arc::new(database))
}

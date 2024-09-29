use dotenv::dotenv;
use mongodb::{bson::doc, error::Result, options::IndexOptions, Client, Database};

use crate::models::projects::ProjectInDB;
use crate::models::work_logs::WorkLogsInDB;

pub async fn init_db() -> Result<Database> {
    dotenv().ok();
    let uri = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let client = Client::with_uri_str(uri).await?;
    let database = client.database("devtrackr_db");
    Ok(database)
}

/// コレクションにインデックスを作成する関数
pub async fn create_indexes(db: &Database) -> Result<()> {
    log::info!("Creating indexes...");
    create_projects_indexes(db).await?;
    create_work_logs_indexes(db).await?;
    log::info!("Indexes created successfully.");
    Ok(())
}

/// projectsコレクションのインデックス作成
async fn create_projects_indexes(db: &Database) -> Result<()> {
    let collection = db.collection::<ProjectInDB>("projects");

    // company_idフィールドにインデックスを作成
    let company_id_index = mongodb::IndexModel::builder()
        .keys(doc! { "company_id": 1 })
        .options(
            IndexOptions::builder()
                .name("idx_company_id".to_string())
                .build(),
        )
        .build();

    // statusフィールドにインデックスを作成
    let status_index = mongodb::IndexModel::builder()
        .keys(doc! { "status": 1 })
        .options(
            IndexOptions::builder()
                .name("idx_status".to_string())
                .build(),
        )
        .build();

    // hourly_payフィールドにインデックスを作成
    let hourly_pay_index = mongodb::IndexModel::builder()
        .keys(doc! { "hourly_pay": 1 })
        .options(
            IndexOptions::builder()
                .name("idx_hourly_pay".to_string())
                .build(),
        )
        .build();

    // skill_labelsフィールドにマルチキーインデックスを作成
    let skill_labels_index = mongodb::IndexModel::builder()
        .keys(doc! { "skill_labels": 1 })
        .options(
            IndexOptions::builder()
                .name("idx_skill_labels".to_string())
                .build(),
        )
        .build();

    collection
        .create_indexes(
            vec![
                company_id_index,
                status_index,
                hourly_pay_index,
                skill_labels_index,
            ],
            None,
        )
        .await?;
    Ok(())
}

/// work_logsコレクションのインデックス作成
async fn create_work_logs_indexes(db: &Database) -> Result<()> {
    let collection = db.collection::<WorkLogsInDB>("work_logs");

    // project_idフィールドにインデックスを作成
    let project_id_index = mongodb::IndexModel::builder()
        .keys(doc! { "project_id": 1 })
        .options(
            IndexOptions::builder()
                .name("idx_project_id".to_string())
                .build(),
        )
        .build();

    collection.create_index(project_id_index, None).await?;
    Ok(())
}

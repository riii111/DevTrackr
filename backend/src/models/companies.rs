use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub enum CompanyStatus {
    PendingContract, // 契約予定
    Contract,        // 契約中
    Completed,       // 完了
    Cancelled,       // キャンセル
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub enum ContractType {
    FullTime,  // 正社員
    PartTime,  // アルバイト
    Contract,  // 契約
    Freelance, // フリーランサー
    SideJob,   // 副業
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AnnualRevenue {
    pub amount: i64,
    pub fiscal_year: i32,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Bonus {
    pub amount: i64,    // 金額(円)
    pub frequency: i32, // 月数で表現（例：6は6ヶ月に1回）
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CompanyInDB {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: Option<ObjectId>,
    #[serde(flatten)]
    pub common: CompanyCommon,
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub affiliation_start_date: BsonDateTime, // 契約開始日
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub affiliation_end_date: Option<BsonDateTime>, // 契約終了日
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub created_at: BsonDateTime,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub updated_at: Option<BsonDateTime>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct CompanyCreate {
    #[serde(flatten)]
    pub common: CompanyCommon,
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub affiliation_start_date: BsonDateTime,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub affiliation_end_date: Option<BsonDateTime>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CompanyUpdate {
    #[serde(flatten)]
    pub common: CompanyCommon,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub affiliation_start_date: Option<BsonDateTime>,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub affiliation_end_date: Option<BsonDateTime>,
}

fn default_company_status() -> CompanyStatus {
    CompanyStatus::PendingContract
}

impl From<CompanyInDB> for CompanyUpdate {
    fn from(company: CompanyInDB) -> Self {
        CompanyUpdate {
            company_name: Some(company.company_name),
            establishment_year: Some(company.establishment_year),
            headquarters_location: Some(company.headquarters_location),
            website_url: Some(company.website_url),
            employee_count: Some(company.employee_count),
            annual_revenue: company.annual_revenue,
            affiliation_start_date: Some(company.affiliation_start_date),
            affiliation_end_date: company.affiliation_end_date,
            contract_type: Some(company.contract_type),
            major_clients: company.major_clients,
            major_services: company.major_services,
            average_hourly_rate: company.average_hourly_rate,
            bonus: company.bonus,
            status: Some(company.status),
        }
    }
}

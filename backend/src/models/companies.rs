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
    pub company_name: String,                  // 企業名
    pub establishment_year: i32,               // 設立年
    pub headquarters_location: String,         // 本社所在地
    pub website_url: String,                   // 企業の公式サイトURL
    pub employee_count: i32,                   // 従業員数
    pub annual_revenue: Option<AnnualRevenue>, // 年間売上
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub affiliation_start_date: BsonDateTime, // 契約開始日
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub affiliation_end_date: Option<BsonDateTime>, // 契約終了日
    pub contract_type: ContractType,           // 契約タイプ
    pub major_clients: Option<Vec<String>>,    // 主要顧客
    pub major_services: Option<Vec<String>>,   // 主要サービス
    pub average_hourly_rate: Option<i32>,      // 平均時給
    pub bonus: Option<Bonus>,                  // ボーナス
    #[serde(default = "default_company_status")]
    pub status: CompanyStatus, // ユーザーとの契約ステータス
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub created_at: BsonDateTime,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub updated_at: Option<BsonDateTime>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct CompanyCreate {
    pub company_name: String,
    pub establishment_year: i32,
    pub headquarters_location: String,
    pub website_url: String,
    pub employee_count: i32,
    pub annual_revenue: Option<AnnualRevenue>,
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub affiliation_start_date: BsonDateTime,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub affiliation_end_date: Option<BsonDateTime>,
    pub contract_type: ContractType,
    pub major_clients: Option<Vec<String>>,
    pub major_services: Option<Vec<String>>,
    pub average_hourly_rate: Option<i32>,
    pub bonus: Option<Bonus>,
    #[serde(default = "default_company_status")]
    pub status: CompanyStatus,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CompanyUpdate {
    pub company_name: Option<String>,
    pub establishment_year: Option<i32>,
    pub headquarters_location: Option<String>,
    pub website_url: Option<String>,
    pub employee_count: Option<i32>,
    pub annual_revenue: Option<AnnualRevenue>,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub affiliation_start_date: Option<BsonDateTime>,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub affiliation_end_date: Option<BsonDateTime>,
    pub contract_type: Option<ContractType>,
    pub major_clients: Option<Vec<String>>,
    pub major_services: Option<Vec<String>>,
    pub average_hourly_rate: Option<i32>,
    pub bonus: Option<Bonus>,
    pub status: Option<CompanyStatus>,
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

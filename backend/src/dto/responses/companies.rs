use crate::models::companies::{AnnualRevenue, Bonus, CompanyInDB, CompanyStatus, ContractType};
use crate::utils::serializer::{
    serialize_bson_datetime, serialize_object_id, serialize_option_bson_datetime,
};
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct CompanyResponse {
    #[serde(serialize_with = "serialize_object_id")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: ObjectId,
    pub company_name: String,
    pub establishment_year: i32,
    pub headquarters_location: String,
    pub website_url: String,
    pub employee_count: i32,
    pub annual_revenue: Option<AnnualRevenue>,
    #[serde(serialize_with = "serialize_bson_datetime")]
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub affiliation_start_date: BsonDateTime,
    #[serde(serialize_with = "serialize_option_bson_datetime")]
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub affiliation_end_date: Option<BsonDateTime>,
    pub contract_type: ContractType,
    pub major_clients: Option<Vec<String>>,
    pub major_services: Option<Vec<String>>,
    pub average_hourly_rate: Option<i32>,
    pub bonus: Option<Bonus>,
    pub status: CompanyStatus,
    #[serde(serialize_with = "serialize_bson_datetime")]
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub created_at: BsonDateTime,
    #[serde(serialize_with = "serialize_option_bson_datetime")]
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub updated_at: Option<BsonDateTime>,
}

impl TryFrom<CompanyInDB> for CompanyResponse {
    type Error = &'static str;

    fn try_from(db_company: CompanyInDB) -> Result<Self, Self::Error> {
        Ok(Self {
            id: db_company.id.ok_or("IDが存在しません")?,
            company_name: db_company.company_name,
            establishment_year: db_company.establishment_year,
            headquarters_location: db_company.headquarters_location,
            website_url: db_company.website_url,
            employee_count: db_company.employee_count,
            annual_revenue: db_company.annual_revenue,
            affiliation_start_date: db_company.affiliation_start_date,
            affiliation_end_date: db_company.affiliation_end_date,
            contract_type: db_company.contract_type,
            major_clients: db_company.major_clients,
            major_services: db_company.major_services,
            average_hourly_rate: db_company.average_hourly_rate,
            bonus: db_company.bonus,
            status: db_company.status,
            created_at: db_company.created_at,
            updated_at: db_company.updated_at,
        })
    }
}

#[derive(Serialize, Debug, ToSchema)]
pub struct CompanyCreatedResponse {
    #[serde(serialize_with = "serialize_object_id")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: ObjectId,
}

impl From<ObjectId> for CompanyCreatedResponse {
    fn from(id: ObjectId) -> Self {
        Self { id }
    }
}

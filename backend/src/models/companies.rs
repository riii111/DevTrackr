use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

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

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
pub struct AnnualRevenue {
    #[validate(range(min = 0, message = "年間売上は0以上である必要があります"))]
    pub amount: i64,
    #[validate(range(max = 2100, message = "会計年度は2100年以前である必要があります"))]
    pub fiscal_year: i32,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
pub struct Bonus {
    #[validate(range(min = 0, message = "ボーナス金額は0以上である必要があります"))]
    pub amount: i64,
    #[validate(range(
        min = 1,
        max = 12,
        message = "ボーナス頻度は1から12の間である必要があります"
    ))]
    pub frequency: i32,
}

// TODO: 共通フィールドの扱い方については要検討。あくまでもモデル実装内部の話なのに、repositoriesやresponseで"common.""と記述するのが面倒
#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
pub struct CompanyCommon {
    #[validate(length(min = 2, max = 100, message = "会社名は2〜100文字である必要があります"))]
    pub company_name: String,
    #[validate(range(
        min = 1800,
        max = 2100,
        message = "設立年は1800年から現在までの間である必要があります"
    ))]
    pub establishment_year: i32,
    #[validate(length(max = 200, message = "本社所在地は200文字以内である必要があります"))]
    pub headquarters_location: String,
    #[validate(url(message = "有効なURLを入力してください"))]
    pub website_url: String,
    #[validate(range(min = 1, message = "従業員数は1以上である必要があります"))]
    pub employee_count: i32,
    pub annual_revenue: Option<AnnualRevenue>,
    pub contract_type: ContractType,
    #[validate(length(max = 10, message = "主要顧客は最大10件まで登録できます"))]
    pub major_clients: Option<Vec<String>>,
    #[validate(length(max = 10, message = "主要サービスは最大10件まで登録できます"))]
    pub major_services: Option<Vec<String>>,
    #[validate(range(
        min = 500,
        max = 100000,
        message = "平均時給は500円から100,000円の間である必要があります"
    ))]
    pub average_hourly_rate: Option<i32>,
    pub bonus: Option<Bonus>,
    pub status: CompanyStatus,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
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

pub trait DateValidator {
    fn validate_dates(&self) -> Result<(), ValidationError>;
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CompanyCreate {
    #[serde(flatten)]
    pub common: CompanyCommon,
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub affiliation_start_date: BsonDateTime,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub affiliation_end_date: Option<BsonDateTime>,
}

impl CompanyCreate {
    fn validate_dates(&self) -> Result<(), ValidationError> {
        let now = BsonDateTime::now();
        if self.affiliation_start_date > now {
            return Err(ValidationError::new(
                "契約開始日は現在日時より前である必要があります",
            ));
        }
        if let Some(end_date) = self.affiliation_end_date {
            if end_date <= self.affiliation_start_date {
                return Err(ValidationError::new(
                    "契約終了日は契約開始日より後である必要があります",
                ));
            }
        }
        Ok(())
    }
}

impl Validate for CompanyCreate {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if let Err(e) = self.common.validate() {
            errors.extend(e);
        }
        if let Err(e) = self.validate_dates() {
            errors.add("dates", e);
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
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

impl CompanyUpdate {
    fn validate_dates(&self) -> Result<(), ValidationError> {
        let now = BsonDateTime::now();
        if let Some(start_date) = self.affiliation_start_date {
            if start_date > now {
                return Err(ValidationError::new(
                    "契約開始日は現在日時より前である必要があります",
                ));
            }
            if let Some(end_date) = self.affiliation_end_date {
                if end_date <= start_date {
                    return Err(ValidationError::new(
                        "契約終了日は契約開始日より後である必要があります",
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Validate for CompanyUpdate {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if let Err(e) = self.common.validate() {
            errors.extend(e);
        }
        if let Err(e) = self.validate_dates() {
            errors.add("dates", e);
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl From<CompanyInDB> for CompanyUpdate {
    fn from(company: CompanyInDB) -> Self {
        CompanyUpdate {
            common: company.common,
            affiliation_start_date: Some(company.affiliation_start_date),
            affiliation_end_date: company.affiliation_end_date,
        }
    }
}

fn default_company_status() -> CompanyStatus {
    CompanyStatus::PendingContract
}

use crate::models::projects::ProjectInDB;
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use chrono::{NaiveDate, TimeZone, Utc};
use chrono_tz::Asia::Tokyo;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use utoipa::ToSchema;
use validator::{Validate, ValidationError, ValidationErrors};

// カスタムバリデーション用のトレイト
trait DateValidator {
    fn validate_dates(&self) -> Result<(), ValidationError>;
}

// バリデーションロジックを共通化するマクロ
macro_rules! impl_company_validation {
    ($type:ty) => {
        impl DateValidator for $type {
            // カスタムバリデーション: 契約開始日と契約終了日のバリデーションを行う
            fn validate_dates(&self) -> Result<(), ValidationError> {
                let today = Tokyo
                    .from_utc_datetime(&Utc::now().naive_utc())
                    .date_naive();

                if self.affiliation_start_date > today {
                    return Err(ValidationError {
                        code: "date_validation".into(),
                        message: Some("契約開始日は現在日付より前である必要があります".into()),
                        params: Default::default(),
                    });
                }
                if let Some(end_date) = self.affiliation_end_date {
                    if end_date <= self.affiliation_start_date {
                        return Err(ValidationError {
                            code: "date_validation".into(),
                            message: Some(
                                "契約終了日は契約開始日より後である必要があります".into(),
                            ),
                            params: Default::default(),
                        });
                    }
                }
                Ok(())
            }
        }

        impl $type {
            pub fn validate_all(&self) -> Result<(), ValidationErrors> {
                let mut errors = ValidationErrors::new();

                // 既存のバリデーションを実行
                if let Err(e) = self.common.validate() {
                    for (field, field_errors) in e.field_errors() {
                        for error in field_errors {
                            errors.add(field, error.clone());
                        }
                    }
                }

                // カスタムバリデーションを実行
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
    };
}

// Validateマクロを使用してCompanyCreateとCompanyUpdateのValidateを実装
impl_company_validation!(CompanyCreate);
impl_company_validation!(CompanyUpdate);

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub enum CompanyStatus {
    #[default]
    PendingContract, // 契約予定
    Contract,  // 契約中
    Completed, // 完了
    Cancelled, // キャンセル
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
pub struct AnnualSales {
    #[validate(range(min = 0, message = "年間売上は0以上である必要があります"))]
    #[schema(example = 100000000)]
    pub amount: i64, // 年間売上

    #[validate(range(max = 2100, message = "会計年度は2100年以前である必要があります"))]
    #[schema(example = 2024)]
    pub fiscal_year: i32, // 会計年度
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
pub struct Bonus {
    #[validate(range(min = 0, message = "ボーナス金額は0以上である必要があります"))]
    #[schema(example = 1000000)]
    pub amount: i64, // ボーナス金額

    #[validate(range(
        min = 1,
        max = 12,
        message = "ボーナス頻度は1から12の間である必要があります"
    ))]
    #[schema(example = 2)]
    pub frequency: i32, // ボーナス頻度
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
pub struct CompanyCommon {
    #[validate(length(min = 2, max = 100, message = "企業名は2〜100文字である必要があります"))]
    #[schema(example = "株式会社テスト")]
    pub company_name: String, // 企業名
    #[validate(range(
        min = 1800,
        max = 2100,
        message = "設立年は1800年から現在までの間である必要があります"
    ))]
    #[schema(example = 2024)]
    pub establishment_year: i32, // 設立年

    #[validate(length(max = 200, message = "所在地は200文字以内である必要があります"))]
    #[schema(example = "東京都千代田区")]
    pub location: String, // 所在地

    #[validate(url(message = "有効なURLを入力してください"))]
    #[schema(example = "https://www.example.com")]
    pub website_url: String, // 企業サイトURL

    #[validate(range(min = 1, message = "従業員数は1以上である必要があります"))]
    #[schema(example = 100)]
    pub employee_count: i32, // 従業員数

    pub annual_sales: Option<AnnualSales>, // 年間売上

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<ContractType>, // 契約タイプ

    #[validate(length(max = 10, message = "主要顧客は最大10件まで登録できます"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_clients: Option<Vec<String>>, // 主要顧客

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 10, message = "主要サービスは最大10件まで登録できます"))]
    pub major_services: Option<Vec<String>>, // 主要サービス

    #[validate(range(
        min = 500,
        max = 100000,
        message = "平均時給は500円から100,000円の間である必要があります"
    ))]
    #[schema(example = 4000)]
    pub average_hourly_rate: Option<i32>, // 平均時給

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = json!({"amount": 100000, "frequency": 1}))]
    pub bonus: Option<Bonus>, // ボーナス

    #[schema(example = "PendingContract")]
    pub status: CompanyStatus, // 契約ステータス
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CompanyInDB {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: Option<ObjectId>,

    #[serde(flatten)]
    pub common: CompanyCommon,

    #[schema(value_type = String, example = "2023-12-01")]
    pub affiliation_start_date: NaiveDate, // 契約開始日

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<String>, example = "2024-09-30")]
    pub affiliation_end_date: Option<NaiveDate>, // 契約終了日

    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub created_at: BsonDateTime, // 作成日時

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub updated_at: Option<BsonDateTime>, // 更新日時
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CompanyWithProjectsInDB {
    pub company: CompanyInDB,

    #[schema(value_type = Vec<ProjectInDB>)]
    pub projects: Vec<ProjectInDB>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CompanyCreate {
    #[serde(flatten)]
    pub common: CompanyCommon,

    /// 契約開始日（JST, YYYY-MM-DD形式で受け取ること）
    #[schema(value_type = String, example = "2023-12-01")]
    pub affiliation_start_date: NaiveDate, // 契約開始日

    /// 契約終了日（JST, YYYY-MM-DD形式で受け取ること）
    #[schema(value_type = Option<String>, example = "2024-09-30")]
    pub affiliation_end_date: Option<NaiveDate>, // 契約終了日
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CompanyUpdate {
    #[serde(flatten)]
    pub common: CompanyCommon,

    /// 契約開始日（JST, YYYY-MM-DD形式で受け取ること）
    #[schema(value_type = String, example = "2023-12-01")]
    pub affiliation_start_date: NaiveDate, // 契約開始日

    /// 契約終了日（JST, YYYY-MM-DD形式で受け取ること）
    #[schema(value_type = Option<String>, example = "2024-09-30")]
    pub affiliation_end_date: Option<NaiveDate>, // 契約終了日
}

impl From<CompanyInDB> for CompanyUpdate {
    fn from(company: CompanyInDB) -> Self {
        CompanyUpdate {
            common: company.common,
            affiliation_start_date: company.affiliation_start_date,
            affiliation_end_date: company.affiliation_end_date,
        }
    }
}

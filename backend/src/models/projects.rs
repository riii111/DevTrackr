use crate::utils::deserializer::{deserialize_skill_labels, deserialize_sort_params};
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnNull};
use std::fmt;
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub enum ProjectStatus {
    #[default]
    Planning, // 企画中
    InProgress, // 進行中
    Completed,  // 完了
    OnHold,     // 一時中断
    Cancelled,  // キャンセル
}

impl fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectStatus::Planning => write!(f, "planning"),
            ProjectStatus::InProgress => write!(f, "in_progress"),
            ProjectStatus::Completed => write!(f, "completed"),
            ProjectStatus::OnHold => write!(f, "on_hold"),
            ProjectStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[derive(Deserialize, Debug, Validate, ToSchema)]
pub struct ProjectCreate {
    #[validate(length(
        min = 1,
        max = 100,
        message = "タイトルは1〜100文字である必要があります"
    ))]
    pub title: String,

    #[validate(length(max = 1000, message = "説明は1000文字以内である必要があります"))]
    pub description: Option<String>,

    #[validate(length(max = 10, message = "スキルラベルは最大10個まで登録できます"))]
    pub skill_labels: Option<Vec<String>>,

    #[schema(value_type = String, example = "70a6c1e9f0f7b9001234abcd")]
    pub company_id: ObjectId, // プロジェクトの企業ID

    #[validate(range(min = 0, message = "時給は0以上である必要があります"))]
    pub hourly_pay: Option<i32>,

    pub status: ProjectStatus,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
pub struct ProjectUpdate {
    #[validate(length(
        min = 1,
        max = 100,
        message = "タイトルは1〜100文字である必要があります"
    ))]
    pub title: String,

    #[validate(length(max = 1000, message = "説明は1000文字以内である必要があります"))]
    pub description: Option<String>,

    #[validate(length(max = 10, message = "スキルラベルは最大10個まで登録できます"))]
    pub skill_labels: Option<Vec<String>>,

    #[schema(value_type = String, example = "70a6c1e9f0f7b9001234abcd")]
    pub company_id: ObjectId, // プロジェクトの企業ID

    #[validate(range(min = 0, message = "時給は0以上である必要があります"))]
    pub hourly_pay: Option<i32>,

    pub status: ProjectStatus,

    #[validate(range(min = 0, message = "総作業時間は0以上である必要があります"))]
    pub total_working_time: i64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ProjectInDB {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: Option<ObjectId>,

    pub title: String, // プロジェクトのタイトル

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>, // プロジェクトの説明

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_labels: Option<Vec<String>>, // プロジェクトの採用技術

    #[schema(value_type = String, example = "70a6c1e9f0f7b9001234abcd")]
    pub company_id: ObjectId, // プロジェクトの企業ID

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_pay: Option<i32>, // プロジェクトの時給

    #[serde_as(as = "DefaultOnNull")]
    pub status: ProjectStatus, // プロジェクトの状況

    pub total_working_time: i64, // 総作業時間

    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub created_at: BsonDateTime, // 作成日時

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub updated_at: Option<BsonDateTime>, // 更新日時
}

impl From<ProjectInDB> for ProjectUpdate {
    fn from(project: ProjectInDB) -> Self {
        ProjectUpdate {
            title: project.title,
            description: project.description,
            skill_labels: project.skill_labels,
            company_id: project.company_id,
            hourly_pay: project.hourly_pay,
            status: project.status,
            total_working_time: project.total_working_time,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct ProjectFilter {
    pub title: Option<String>,

    pub status: Option<String>,

    pub skill_labels: Option<Vec<String>>,

    pub company_id: Option<ObjectId>,
}

impl ProjectFilter {
    /// フィルタが空かどうかをチェックする
    pub fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.status.is_none()
            && self.skill_labels.is_none()
            && self.company_id.is_none()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Deserialize, ToSchema, Validate, Clone)]
pub struct ProjectQuery {
    /// プロジェクトのタイトル（部分一致）
    #[schema(example = "プロジェクトA")]
    pub title: Option<String>,

    /// プロジェクトのステータス
    #[schema(example = "InProgress")]
    pub status: Option<String>,

    /// スキルラベルの一覧
    #[serde(default, deserialize_with = "deserialize_skill_labels")]
    #[schema(example = json!(["C++", "Rust"]), value_type = Vec<String>)]
    #[validate(length(max = 10, message = "スキルラベルは最大10個まで指定できます"))]
    pub skill_labels: Option<Vec<String>>,

    /// 企業ID
    #[schema(value_type = String, example = "80a6c1e9f0f7b9001234abcd")]
    pub company_id: Option<ObjectId>,

    /// 取得するドキュメント数の制限
    #[schema(example = 10)]
    #[validate(range(min = 1, max = 100, message = "limitは1から100の間で指定してください"))]
    pub limit: Option<i64>,

    /// 取得を開始する位置
    #[schema(example = 0)]
    #[validate(range(min = 0, message = "offsetは0以上で指定してください"))]
    pub offset: Option<u64>,

    /// ソート条件（例: "name:asc", "created_at:desc"）
    #[serde(default, deserialize_with = "deserialize_sort_params")]
    #[schema(example = json!(["title:asc", "created_at:desc"]), value_type = Vec<String>)]
    #[validate(custom(function = "validate_sort_params"))]
    pub sort: Option<Vec<String>>,
}

impl ProjectQuery {
    /// QueryパラメータからProjectFilterへの変換を行う
    pub fn into_filter(self) -> Option<ProjectFilter> {
        let filter = ProjectFilter {
            title: self.title,
            status: self.status,
            skill_labels: self.skill_labels,
            company_id: self.company_id,
        };

        if filter.is_empty() {
            None
        } else {
            Some(filter)
        }
    }

    /// ソートパラメータを MongoDB 用の形式に変換する
    pub fn parse_sort_params(&self) -> Option<Vec<(String, i8)>> {
        self.sort.as_ref().map(|sort_params| {
            sort_params
                .iter()
                .filter_map(|param| {
                    let parts: Vec<&str> = param.split(':').collect();
                    if parts.len() == 2 {
                        Some((
                            parts[0].to_string(),
                            if parts[1].to_lowercase() == "asc" {
                                1
                            } else {
                                -1
                            },
                        ))
                    } else {
                        None
                    }
                })
                .collect()
        })
    }
}

/// ソートパラメータのバリデーション
///
/// # バリデーションルール
/// - ソートキーは許可されたフィールドのみ
/// - ソート順は "asc" または "desc" のみ
///
/// # エラー処理方針
/// クライアントサイドのバグを早期に検出するため、無効な値は400エラーを返す。
/// これにより、APIの正しい使用方法を強制し、予期せぬ動作を防ぐ。
fn validate_sort_params(sort: &[String]) -> Result<(), ValidationError> {
    // 許可されたソートフィールド
    const ALLOWED_FIELDS: [&str; 4] = ["title", "status", "created_at", "updated_at"];

    for param in sort {
        let parts: Vec<&str> = param.split(':').collect();
        if parts.len() != 2 {
            let mut err = ValidationError::new("sort_format");
            err.message = Some("Invalid sort format. Expected 'field:order'".into());
            return Err(err);
        }

        let field = parts[0];
        let order = parts[1].to_lowercase();

        // フィールドの検証
        if !ALLOWED_FIELDS.contains(&field) {
            let mut err = ValidationError::new("sort_field");
            err.message = Some(
                format!(
                    "Invalid sort field: {}. Allowed fields are: {:?}",
                    field, ALLOWED_FIELDS
                )
                .into(),
            );
            return Err(err);
        }

        // ソート順の検証
        if order != "asc" && order != "desc" {
            let mut err = ValidationError::new("sort_order");
            err.message = Some("Sort order must be either 'asc' or 'desc'".into());
            return Err(err);
        }
    }
    Ok(())
}

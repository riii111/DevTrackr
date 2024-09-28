use crate::dto::responses::projects::{ProjectCreatedResponse, ProjectResponse};
use crate::errors::app_error::AppError;
use crate::models::projects::{ProjectCreate, ProjectFilter, ProjectUpdate};
use crate::repositories::projects::MongoProjectRepository;
use crate::usecases::projects::ProjectUseCase;
use actix_web::{get, post, put, web, HttpResponse};
use bson::oid::ObjectId;
use log::info;
use serde::Deserialize;
use std::sync::Arc;
use validator::Validate;

#[derive(Deserialize)]
pub struct ProjectQuery {
    /// プロジェクトのタイトル（部分一致）
    title: Option<String>,
    /// プロジェクトのステータス
    status: Option<String>,
    /// スキルラベルの一覧
    skill_labels: Option<Vec<String>>,
    /// 取得するドキュメント数の制限
    limit: Option<i64>,
    /// 取得を開始する位置
    offset: Option<u64>,
    /// ソート条件（例: "name:asc", "created_at:desc"）
    sort: Option<Vec<String>>,
}

#[utoipa::path(
    get,
    path = "/projects",
    params(
        ("title" = Option<String>, Query, description = "プロジェクトのタイトル（部分一致）"),
        ("status" = Option<String>, Query, description = "プロジェクトのステータス"),
        ("skill_labels" = Option<Vec<String>>, Query, description = "スキルラベルの一覧"),
        ("limit" = Option<i64>, Query, description = "取得するドキュメント数の制限"),
        ("offset" = Option<u64>, Query, description = "取得を開始する位置"),
        ("sort" = Option<Vec<String>>, Query, description = "ソート条件（例: 'name:asc', 'created_at:desc'）")
    ),
    responses(
        (status = 200, description = "プロジェクトの取得に成功", body = Vec<ProjectResponse>),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    )
)]
#[get("")]
pub async fn get_projects(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    query: web::Query<ProjectQuery>,
) -> Result<HttpResponse, AppError> {
    info!("called GET search_projects!!");

    // クエリパラメータを ProjectFilter にマッピング
    let filter = ProjectFilter {
        title: query.title.clone(),
        status: query.status.clone(),
        skill_labels: query.skill_labels.clone(),
    };

    // ソート条件のパース
    let sort = if let Some(sort_params) = &query.sort {
        let parsed_sort = sort_params
            .iter()
            .filter_map(|s| {
                let parts: Vec<&str> = s.split(':').collect();
                if parts.len() == 2 {
                    let key = parts[0].to_string();
                    let order = match parts[1].to_lowercase().as_str() {
                        "asc" => 1,
                        "desc" => -1,
                        _ => return None,
                    };
                    Some((key, order))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, i8)>>();

        if parsed_sort.is_empty() {
            None
        } else {
            Some(parsed_sort)
        }
    } else {
        None
    };

    let projects = usecase
        .search_projects(
            if filter.is_empty() {
                None
            } else {
                Some(filter)
            },
            query.limit,
            query.offset,
            sort,
        )
        .await?;

    let response = projects
        .into_iter()
        .map(ProjectResponse::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| AppError::InternalServerError(format!("データの変換に失敗しました: {}", e)))?;

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/projects/{id}",
    responses(
        (status = 200, description = "プロジェクトの取得に成功", body = ProjectResponse),
        (status = 400, description = "無効なIDです", body = ErrorResponse),
        (status = 404, description = "プロジェクトが見つかりません", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "プロジェクトID")
    )
)]
#[get("/{id}")]
pub async fn get_project_by_id(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    info!("called GET get_project_by_id!!");

    let project = match usecase.get_project_by_id(&id).await {
        Ok(Some(project)) => project,
        Ok(None) => {
            return Err(AppError::NotFound(
                "プロジェクトが見つかりません".to_string(),
            ))
        }
        Err(e) => return Err(e), // AppErrorをそのまま返す
    };

    let response = ProjectResponse::try_from(project)
        .map_err(|e| AppError::InternalServerError(format!("データの変換に失敗しました: {}", e)))?;

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    post,
    path = "/projects",
    request_body = ProjectCreate,
    responses(
        (status = 201, description = "プロジェクトの作成に成功", body = ProjectCreatedResponse),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    )
)]
#[post("")]
pub async fn create_project(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    project: web::Json<ProjectCreate>,
) -> Result<HttpResponse, AppError> {
    info!("called POST create_project!!");

    // バリデーションを実行
    project
        .validate()
        .map_err(|e| AppError::ValidationError(e))?;

    let project_id = usecase.create_project(project.into_inner()).await?;

    Ok(HttpResponse::Created().json(ProjectCreatedResponse::from(project_id)))
}

#[utoipa::path(
    put,
    path = "/projects/{id}",
    request_body = ProjectUpdate,
    responses(
        (status = 204, description = "プロジェクトの更新に成功"),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 404, description = "プロジェクトが見つかりません", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "プロジェクトID")
    )
)]
#[put("/{id}")]
pub async fn update_project_by_id(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    path: web::Path<String>,
    project: web::Json<ProjectUpdate>,
) -> Result<HttpResponse, AppError> {
    info!("called PUT update_project_by_id!!");

    let obj_id = ObjectId::parse_str(&path.into_inner())
        .map_err(|_| AppError::BadRequest("無効なIDです".to_string()))?;

    // バリデーションチェック
    project
        .validate()
        .map_err(|e| AppError::ValidationError(e))?;

    usecase
        .update_project(&obj_id, &project.into_inner())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

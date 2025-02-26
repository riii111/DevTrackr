use crate::dto::responses::projects::{ProjectCreatedResponse, ProjectResponse};
use crate::errors::app_error::AppError;
use crate::models::projects::{ProjectCreate, ProjectQuery, ProjectUpdate};
use crate::repositories::projects::MongoProjectRepository;
use crate::usecases::projects::ProjectUseCase;
use actix_web::{get, post, put, web, HttpResponse};
use bson::oid::ObjectId;
use log::info;
use std::sync::Arc;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/api/projects/",
    params(
        ("title" = Option<String>, Query, description = "プロジェクトのタイトル（部分一致）"),
        ("status" = Option<String>, Query, description = "プロジェクトのステータス"),
        ("skill_labels" = Option<Vec<String>>, Query, description = "スキルラベルの一覧"),
        ("company_id" = Option<String>, Query, description = "企業ID"),
        ("limit" = Option<i64>, Query, description = "取得するドキュメント数の制限"),
        ("offset" = Option<u64>, Query, description = "取得を開始する位置(0スタート)"),
        ("sort" = Option<Vec<String>>, Query, description = "ソート条件（例: 'name:asc', 'created_at:desc'）")
    ),
    responses(
        (status = 200, description = "プロジェクトの取得に成功", body = Vec<ProjectResponse>),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
#[get("/")]
pub async fn get_projects(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    query: web::Query<ProjectQuery>,
) -> Result<HttpResponse, AppError> {
    info!("called GET search_projects!!");

    // バリデーションを実行
    query.validate().map_err(AppError::ValidationError)?;

    let query_inner = query.into_inner();
    let projects = usecase
        .search_projects(
            query_inner.clone().into_filter(),
            query_inner.limit,
            query_inner.offset,
            query_inner.parse_sort_params(),
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
    path = "/api/projects/{id}/",
    responses(
        (status = 200, description = "プロジェクトの取得に成功", body = ProjectResponse),
        (status = 400, description = "無効なIDです", body = ErrorResponse),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 404, description = "プロジェクトが見つかりません", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "プロジェクトID")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/{id}/")]
pub async fn get_project_by_id(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    info!("called GET get_project_by_id!!");

    let obj_id = ObjectId::parse_str(id.into_inner())
        .map_err(|_| AppError::BadRequest("無効なIDです".to_string()))?;

    let project = match usecase.get_project_by_id(&obj_id).await {
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
    path = "/api/projects/",
    request_body = ProjectCreate,
    responses(
        (status = 201, description = "プロジェクトの作成に成功", body = ProjectCreatedResponse),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/")]
pub async fn create_project(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    create_dto: web::Json<ProjectCreate>,
) -> Result<HttpResponse, AppError> {
    info!("called POST create_project!!");

    // バリデーションを実行
    create_dto.validate().map_err(AppError::ValidationError)?;

    let project_id = usecase.create_project(create_dto.into_inner()).await?;

    Ok(HttpResponse::Created().json(ProjectCreatedResponse::from(project_id)))
}

#[utoipa::path(
    put,
    path = "/api/projects/{id}/",
    request_body = ProjectUpdate,
    responses(
        (status = 204, description = "プロジェクトの更新に成功"),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 404, description = "プロジェクトが見つかりません", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "プロジェクトID")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[put("/{id}/")]
pub async fn update_project_by_id(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    path: web::Path<String>,
    update_dto: web::Json<ProjectUpdate>,
) -> Result<HttpResponse, AppError> {
    info!("called PUT update_project_by_id!!");

    let obj_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| AppError::BadRequest("無効なIDです".to_string()))?;

    // バリデーションチェック
    update_dto.validate().map_err(AppError::ValidationError)?;

    usecase
        .update_project(&obj_id, &update_dto.into_inner())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

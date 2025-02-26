use crate::clients::aws_s3::S3Client;
use crate::repositories::auth::MongoAuthRepository;
use crate::repositories::companies::MongoCompanyRepository;
use crate::repositories::projects::MongoProjectRepository;
use crate::repositories::work_logs::MongoWorkLogRepository;
use crate::usecases::auth::AuthUseCase;
use crate::usecases::companies::CompanyUseCase;
use crate::usecases::projects::ProjectUseCase;
use crate::usecases::work_logs::WorkLogUseCase;
use mongodb::Database;
use std::env;
use std::sync::Arc;

// work_logs
pub fn init_work_logs_usecase(
    db: &Database,
    project_usecase: Arc<ProjectUseCase<MongoProjectRepository>>,
) -> Arc<WorkLogUseCase<MongoWorkLogRepository>> {
    let work_logs_repository = Arc::new(MongoWorkLogRepository::new(db));
    Arc::new(WorkLogUseCase::new(work_logs_repository, project_usecase))
}

// project
pub fn init_project_usecase(
    db: &Database,
    company_usecase: Arc<CompanyUseCase<MongoCompanyRepository>>,
) -> Arc<ProjectUseCase<MongoProjectRepository>> {
    let project_repository = Arc::new(MongoProjectRepository::new(db));
    Arc::new(ProjectUseCase::new(project_repository, company_usecase))
}

// company
pub fn init_company_usecase(db: &Database) -> Arc<CompanyUseCase<MongoCompanyRepository>> {
    let company_repository = Arc::new(MongoCompanyRepository::new(db));
    Arc::new(CompanyUseCase::new(company_repository))
}

// auth
pub fn init_auth_usecase(
    db: &Database,
    s3_client: Arc<S3Client>,
) -> Arc<AuthUseCase<MongoAuthRepository>> {
    let auth_repository = Arc::new(MongoAuthRepository::new(db));

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRETが設定されていません")
        .into_bytes();
    Arc::new(AuthUseCase::new(auth_repository, &jwt_secret, s3_client))
}

use crate::repositories::projects::MongoProjectRepository;
use crate::repositories::work_logs::MongoWorkLogsRepository;
use crate::usecases::projects::ProjectUseCase;
use crate::usecases::work_logs::WorkLogsUseCase;
use mongodb::Database;
use std::sync::Arc;

// work_logs
pub fn init_work_logs_usecase(
    db: &Database,
    project_usecase: Arc<ProjectUseCase<MongoProjectRepository>>,
) -> Arc<WorkLogsUseCase<MongoWorkLogsRepository>> {
    let work_logs_repository = Arc::new(MongoWorkLogsRepository::new(db));
    Arc::new(WorkLogsUseCase::new(work_logs_repository, project_usecase))
}

// project
pub fn init_project_usecase(db: &Database) -> Arc<ProjectUseCase<MongoProjectRepository>> {
    let project_repository = Arc::new(MongoProjectRepository::new(db));
    Arc::new(ProjectUseCase::new(project_repository))
}

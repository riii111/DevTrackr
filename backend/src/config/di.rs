use crate::repositories::projects::MongoProjectRepository;
use crate::usecases::projects::ProjectUseCase;
use mongodb::Database;
use std::sync::Arc;

pub struct AppState {
    pub project_usecase: Arc<ProjectUseCase<MongoProjectRepository>>,
    // pub working_time_usecase: Arc<WorkingTimeUseCase>,
}

pub fn init_dependencies(db: &Database) -> Arc<AppState> {
    let project_repository = Arc::new(MongoProjectRepository::new(db));

    Arc::new(AppState {
        project_usecase: Arc::new(ProjectUseCase::new(project_repository)),
        // 他のユースケース...
    })
}

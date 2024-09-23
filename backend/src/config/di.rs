use crate::repositories::projects::MongoProjectRepository;
use crate::repositories::working_times::MongoWorkingTimeRepository;
use crate::usecases::projects::ProjectUseCase;
use crate::usecases::working_times::WorkingTimeUseCase;
use mongodb::Database;
use std::sync::Arc;

// working_time
pub fn init_working_time_usecase(
    db: Arc<Database>,
) -> Arc<WorkingTimeUseCase<MongoWorkingTimeRepository>> {
    let working_time_repository = Arc::new(MongoWorkingTimeRepository::new(db));
    Arc::new(WorkingTimeUseCase::new(working_time_repository))
}

// project
pub fn init_project_usecase(db: Arc<Database>) -> Arc<ProjectUseCase<MongoProjectRepository>> {
    let project_repository = Arc::new(MongoProjectRepository::new(db));
    Arc::new(ProjectUseCase::new(project_repository))
}

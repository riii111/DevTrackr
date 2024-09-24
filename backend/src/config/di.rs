use crate::adapters::async_queue::{AsyncQueueAdapter, UpdateEvent};
use crate::repositories::projects::MongoProjectRepository;
use crate::repositories::working_times::MongoWorkingTimeRepository;
use crate::usecases::projects::ProjectUseCase;
use crate::usecases::working_times::WorkingTimeUseCase;
use mongodb::Database;
use std::sync::Arc;
use tokio::sync::mpsc;

pub fn init_async_queue() -> (AsyncQueueAdapter, mpsc::Receiver<UpdateEvent>) {
    let (sender, receiver) = mpsc::channel(100);
    let queue_adapter = AsyncQueueAdapter::new(sender);
    (queue_adapter, receiver)
}

// working_time
pub fn init_working_time_usecase(
    db: Arc<Database>,
    queue_adapter: AsyncQueueAdapter,
) -> Arc<WorkingTimeUseCase<MongoWorkingTimeRepository>> {
    let working_time_repository = Arc::new(MongoWorkingTimeRepository::new(db));
    Arc::new(WorkingTimeUseCase::new(
        working_time_repository,
        queue_adapter,
    ))
}

// project
pub fn init_project_usecase(db: Arc<Database>) -> Arc<ProjectUseCase<MongoProjectRepository>> {
    let project_repository = Arc::new(MongoProjectRepository::new(db));
    Arc::new(ProjectUseCase::new(project_repository))
}

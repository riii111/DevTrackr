use bson::oid::ObjectId;
use tokio::sync::mpsc;

pub enum UpdateEvent {
    WorkingTimeCreated(ObjectId, i64),
    WorkingTimeUpdated(ObjectId, i64),
    // WorkingTimeDeleted(ObjectId, i64),  // TODO: 削除APIを追加後に対応
}

pub struct AsyncQueueAdapter {
    sender: mpsc::Sender<UpdateEvent>,
}

impl AsyncQueueAdapter {
    pub fn new(sender: mpsc::Sender<UpdateEvent>) -> Self {
        Self { sender }
    }

    pub async fn enqueue(
        &self,
        project_id: ObjectId,
        duration: i64,
    ) -> Result<(), mpsc::error::SendError<UpdateEvent>> {
        self.sender
            .send(UpdateEvent::WorkingTimeCreated(project_id, duration))
            .await
    }
}

pub async fn run_async_queue_worker(mut receiver: mpsc::Receiver<UpdateEvent>) {
    while let Some(event) = receiver.recv().await {
        match event {
            UpdateEvent::WorkingTimeCreated(project_id, duration) => {
                log::info!(
                    "WorkingTimeCreated: project_id: {}, duration: {}",
                    project_id,
                    duration
                );
                // プロジェクトの総稼働時間を更新するロジック
            }
            UpdateEvent::WorkingTimeUpdated(project_id, duration_diff) => {
                log::info!(
                    "WorkingTimeUpdated: project_id: {}, duration_diff: {}",
                    project_id,
                    duration_diff
                );
                // プロジェクトの総稼働時間を更新するロジック
            } // TODO: 削除APIを追加後に対応
              // UpdateEvent::WorkingTimeDeleted(project_id, duration) => {
              // プロジェクトの総稼働時間を更新するロジック
              // }
        }
    }
}

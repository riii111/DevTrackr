// use crate::models::projects::ProjectInDB;
// use tokio::sync::mpsc;

// pub enum UpdateEvent {
//     WorkingTimeCreated(ProjectInDB, i64),
//     WorkingTimeUpdated(ProjectInDB, i64),
//     WorkingTimeDeleted(ProjectInDB, i64), // TODO: 削除APIを追加後に対応
// }

// pub struct AsyncQueueAdapter {
//     sender: mpsc::Sender<UpdateEvent>,
// }

// impl AsyncQueueAdapter {
//     pub fn new(sender: mpsc::Sender<UpdateEvent>) -> Self {
//         Self { sender }
//     }

//     pub async fn enqueue(
//         &self,
//         project: ProjectInDB,
//         duration: i64,
//     ) -> Result<(), mpsc::error::SendError<UpdateEvent>> {
//         self.sender
//             .send(UpdateEvent::WorkingTimeCreated(project, duration))
//             .await
//     }
// }

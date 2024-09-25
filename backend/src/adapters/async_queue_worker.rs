// use crate::repositories::working_times::WorkingTimeRepository;
// use crate::usecases::working_times::WorkingTimeUseCase;
// use std::sync::Arc;
// use tokio::sync::mpsc;

// use crate::adapters::async_queue_adapter::UpdateEvent;

// pub async fn run_async_queue_worker<R: WorkingTimeRepository>(
//     mut receiver: mpsc::Receiver<UpdateEvent>,
//     working_time_usecase: Arc<WorkingTimeUseCase<R>>,
// ) {
//     while let Some(event) = receiver.recv().await {
//         match event {
//             UpdateEvent::WorkingTimeCreated(project, duration) => {
//                 log::info!(
//                     "WorkingTimeCreated: project_id: {}, duration: {}",
//                     project.id.unwrap(),
//                     duration
//                 );
//                 // プロジェクトの総稼働時間を再計算
//                 if let Err(e) = working_time_usecase
//                     .update_total_working_time(&project)
//                     .await
//                 {
//                     log::error!("Failed to update total working time: {}", e);
//                 }
//             }
//             UpdateEvent::WorkingTimeUpdated(project, duration_diff) => {
//                 log::info!(
//                     "WorkingTimeUpdated: project_id: {}, duration_diff: {}",
//                     project.id.unwrap(),
//                     duration_diff
//                 );
//                 // プロジェクトの総稼働時間を再計算
//                 if let Err(e) = working_time_usecase
//                     .update_total_working_time(&project)
//                     .await
//                 {
//                     log::error!("Failed to update total working time: {}", e);
//                 }
//             } // TODO: 削除APIを追加後に対応
//               // UpdateEvent::WorkingTimeDeleted(project, duration) => {
//               // プロジェクトの総稼働時間を再計算
//               // }
//         }
//     }
// }

use crate::middleware::auth::auth_middleware;
use crate::{
    models::{AppState, QuestTaskDocument,CompletedTaskDocument},
    utils::get_error,
};
use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use axum_auto_routes::route;
use futures::StreamExt;
use mongodb::bson::doc;
use serde::Deserialize;
use std::sync::Arc;
use serde_json::json;
use std::collections::HashSet;

#[derive(Deserialize)]
pub struct GetQuestUsersParams {
    quest_id: i64,
}

#[route(get, "/admin/quests/get_quest_users", auth_middleware)]
pub async fn get_quest_users_handler(
    State(state): State<Arc<AppState>>,
    Extension(_sub): Extension<String>, // Authentication handled by middleware
    Query(params): Query<GetQuestUsersParams>,
) -> impl IntoResponse {
    let tasks_collection = state.db.collection::<QuestTaskDocument>("tasks");
    let completed_tasks_collection = state.db.collection::<CompletedTaskDocument>("completed_tasks");

    // Fetch all task IDs for the given quest_id
    let task_filter = doc! { "quest_id": params.quest_id };
    let task_cursor = match tasks_collection.find(task_filter, None).await {
        Ok(cursor) => cursor,
        Err(e) => return get_error(format!("Error fetching tasks: {}", e)),
    };

    let task_ids_result: Result<Vec<i64>, _> = task_cursor
        .map_ok(|doc| doc.id)
        .collect()
        .await;

    let task_ids = match task_ids_result {
        Ok(ids) => ids,
        Err(e) => return get_error(format!("Error processing tasks: {}", e)),
    };

    if task_ids.is_empty() {
        return get_error(format!("No tasks found for quest_id {}", params.quest_id));
    }

    // Fetch all completed tasks for these task IDs
    let completed_task_filter = doc! { "task_id": { "$in": &task_ids } };
    let completed_task_cursor = match completed_tasks_collection.find(completed_task_filter, None).await {
        Ok(cursor) => cursor,
        Err(e) => return get_error(format!("Error fetching completed tasks: {}", e)),
    };

    let completed_tasks_result: Result<Vec<CompletedTaskDocument>, _> = completed_task_cursor.try_collect().await;

    let users = match completed_tasks_result {
        Ok(completed_tasks) => {
            let user_set: HashSet<String> = completed_tasks.into_iter()
                .map(|task| task.address)
                .collect();

            let users_list: Vec<String> = user_set.into_iter().collect();
            users_list
        }
        Err(e) => return get_error(format!("Error processing completed tasks: {}", e)),
    };

    (
        StatusCode::OK,
        Json(json!({ "users": users })).into_response(),
    )
}
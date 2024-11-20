use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use axum_auto_routes::route;
use futures::{StreamExt, TryStreamExt};
use mongodb::bson::doc;
use serde::Deserialize;
use serde_json::json;

use crate::endpoints::admin::user;
use crate::middleware::auth::auth_middleware;
use crate::{
    models::{AppState, CompletedTaskDocument, QuestTaskDocument},
    utils::get_error,
};

pub_struct!(Deserialize; GetQuestParticipantsParams {
    quest_id: i64,
});

#[route(get, "/admin/quests/get_quest_participants")]
pub async fn get_quest_participants_handler(
    State(state): State<Arc<AppState>>,
    //Extension(_sub): Extension<String>, // Assuming admin authentication is handled
    Query(params): Query<GetQuestParticipantsParams>,
) -> impl IntoResponse {
    let tasks_collection = state.db.collection::<QuestTaskDocument>("tasks");
    let completed_tasks_collection = state
        .db
        .collection::<CompletedTaskDocument>("completed_tasks");

    // Fetch all task IDs for the given quest_id
    let task_filter = doc! { "quest_id": params.quest_id };
    let mut task_cursor = match tasks_collection.find(task_filter, None).await {
        Ok(cursor) => cursor,
        Err(e) => return get_error(format!("Error fetching tasks: {}", e)),
    };

    let mut task_ids = Vec::new();
    while let Some(doc) = task_cursor.next().await {
        match doc {
            Ok(task) => task_ids.push(task.id),
            Err(e) => return get_error(format!("Error processing tasks: {}", e)),
        }
    }

    if task_ids.is_empty() {
        return get_error(format!("No tasks found for quest_id {}", params.quest_id));
    }

    let completed_task_filter = doc! { "task_id": { "$in": &task_ids } };
    let mut completed_task_cursor = match completed_tasks_collection
        .find(completed_task_filter, None)
        .await
    {
        Ok(cursor) => cursor,
        Err(e) => return get_error(format!("Error fetching completed tasks: {}", e)),
    };

    let mut address_task_map: HashMap<String, HashSet<i64>> = HashMap::new();

    while let Some(doc) = completed_task_cursor.next().await {
        match doc {
            Ok(task) => {
                address_task_map
                    .entry(task.address.clone())
                    .or_insert_with(HashSet::new)
                    .insert(task.task_id.into());
            }
            Err(e) => return get_error(format!("Error processing completed tasks: {}", e)),
        }
    }

    // Filter addresses who have completed all tasks
    let total_tasks = task_ids.len();
    let participants: Vec<String> = address_task_map
        .into_iter()
        .filter_map(|(address, tasks_completed)| {
            if tasks_completed.len() == total_tasks {
                Some(address)
            } else {
                None
            }
        })
        .collect();

    let participants_json = json!({ "participants": participants });
    (StatusCode::OK, Json(participants_json)).into_response()
}

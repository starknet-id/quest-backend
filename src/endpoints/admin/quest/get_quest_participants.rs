use std::sync::Arc;

use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use axum_auto_routes::route;
use futures::StreamExt;
use mongodb::bson::doc;
use serde::Deserialize;
use serde_json::json;
use starknet::core::types::FieldElement;

use crate::{middleware::auth::auth_middleware, utils::to_hex};
use crate::{
    models::{AppState, CompletedTaskDocument, QuestTaskDocument},
    utils::get_error,
};

pub_struct!(Deserialize; GetQuestParticipantsParams {
    quest_id: i64,
});

#[route(get, "/admin/quests/get_quest_participants", auth_middleware)]
pub async fn get_quest_participants_handler(
    State(state): State<Arc<AppState>>,
    Extension(_sub): Extension<String>,
    Query(params): Query<GetQuestParticipantsParams>,
) -> impl IntoResponse {
    let tasks_collection = state.db.collection::<QuestTaskDocument>("tasks");
    let completed_tasks_collection = state
        .db
        .collection::<CompletedTaskDocument>("completed_tasks");

    // Fetch all task IDs for the given quest_id
    let task_filter = doc! { "quest_id": params.quest_id };
    let task_ids: Vec<i32> = match tasks_collection.find(task_filter, None).await {
        Ok(mut cursor) => {
            let mut ids = Vec::new();
            while let Some(doc) = cursor.next().await {
                match doc {
                    Ok(task) => ids.push(task.id),
                    Err(e) => return get_error(format!("Error processing tasks: {}", e)),
                }
            }
            ids
        }
        Err(e) => return get_error(format!("Error fetching tasks: {}", e)),
    };

    if task_ids.is_empty() {
        return get_error(format!("No tasks found for quest_id {}", params.quest_id));
    }

    // Use aggregation pipeline to fetch completed tasks and group by address
    let pipeline = vec![
        doc! { "$match": { "task_id": { "$in": &task_ids } } },
        doc! { "$group": {
            "_id": "$address",
            "task_ids": { "$addToSet": "$task_id" },
            "max_timestamp": { "$max": "$timestamp" }
        }},
        doc! { "$project": {
            "address": "$_id",
            "tasks_completed_count": { "$size": "$task_ids" },
            "quest_completion_timestamp": "$max_timestamp"
        }},
    ];

    let mut cursor = match completed_tasks_collection.aggregate(pipeline, None).await {
        Ok(cursor) => cursor,
        Err(e) => return get_error(format!("Error aggregating completed tasks: {}", e)),
    };

    let total_tasks = task_ids.len();
    let mut participants = Vec::new();

    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(doc) => {
                // Get the decimal address and convert it to a hex string
                let address: String = match doc.get_str("address") {
                    Ok(addr) => to_hex(FieldElement::from_dec_str(addr).unwrap()),
                    Err(_) => continue, // Skip invalid documents
                };

                let tasks_completed_count: usize = match doc.get_i32("tasks_completed_count") {
                    Ok(count) => count as usize,
                    Err(_) => continue, // Skip invalid documents
                };

                if tasks_completed_count == total_tasks {
                    participants.push(address);
                }
            }
            Err(e) => return get_error(format!("Error processing aggregation results: {}", e)),
        }
    }

    let participants_json = json!({ "participants": participants });
    (StatusCode::OK, Json(participants_json)).into_response()
}


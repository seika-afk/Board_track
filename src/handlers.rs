use std::sync::Arc;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{AppState,model::GameModel,schema::GameSchema};

pub async fn create_game_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<GameSchema>,
) -> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)>{

    let id = uuid::Uuid::new_v4();

    let game = sqlx::query_as!(
        GameModel,
        r#"INSERT INTO games (id,name,creator,plays) VALUES ($1,$2,$3,$4) RETURNING *"#,
        &id,
        &body.name,
        &body.creator,
        &body.plays
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| e.to_string());

    if let Err(err) = game {
        if err.to_string().contains("duplicate key value") {
            let error_response = json!({
                "status": "error",
                "message": "Game already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    let game = game.unwrap(); 

    let game_resp = json!({
        "status":"success",
        "data":{
            "game": game
        }
    });

    Ok(Json(game_resp))
}

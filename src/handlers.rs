use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    AppState,
    model::GameModel,
    schema::{GameSchema, UpdateGameSchema},
};

pub async fn create_game_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<GameSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4();

    let inserted = sqlx::query_as::<_, GameModel>(
        r#"INSERT INTO games (id,name,creator,plays) VALUES ($1,$2,$3,$4) RETURNING *"#,
    )
    .bind(&id)
    .bind(&body.name)
    .bind(&body.creator)
    .bind(&body.plays)
    .fetch_one(&data.db)
    .await;

    match inserted {
        Ok(game) => {
            let game_resp = json!({
                "status": "success",
                "data": { "game": game }
            });
            Ok(Json(game_resp))
        }
        Err(err) => {
            let err_message = err.to_string();
            if err_message.contains("duplicate key value") {
                let error_response = json!({
                    "status": "error",
                    "message": "Game already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": err_message,
                })),
            ))
        }
    }
}

pub async fn game_list_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let games = sqlx::query_as::<_, GameModel>(r#"SELECT * FROM games ORDER BY name"#)
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Database error: {}", e),
                })),
            )
        })?;

    let json_response = json!({
        "status": "success",
        "count": games.len(),
        "games": games,
    });

    Ok(Json(json_response))
}

pub async fn get_game_handler(
    Path(game_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let game = sqlx::query_as::<_, GameModel>(r#"SELECT * FROM games WHERE id = $1"#)
        .bind(&game_id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": "error",
                    "message": "Game not found",
                })),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e),
                })),
            ),
        })?;

    Ok(Json(json!({
        "status": "success",
        "data": { "game": game },
    })))
}

pub async fn update_game_handler(
    Path(game_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
    Json(payload): Json<UpdateGameSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let existing = sqlx::query_as::<_, GameModel>(r#"SELECT * FROM games WHERE id = $1"#)
        .bind(&game_id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": "error",
                    "message": "Game not found",
                })),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e),
                })),
            ),
        })?;

    let name = payload.name.unwrap_or_else(|| existing.name.clone());
    let creator = payload.creator.unwrap_or_else(|| existing.creator.clone());
    let plays = payload.plays.unwrap_or(existing.plays);

    let updated = sqlx::query_as::<_, GameModel>(
        r#"UPDATE games SET name = $1, creator = $2, plays = $3 WHERE id = $4 RETURNING *"#,
    )
    .bind(&name)
    .bind(&creator)
    .bind(plays)
    .bind(&game_id)
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("{:?}", e),
            })),
        )
    })?;

    Ok(Json(json!({
        "status": "success",
        "data": { "game": updated },
    })))
}

pub async fn delete_game_handler(
    Path(game_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result =
        sqlx::query_as::<_, GameModel>(r#"DELETE FROM games WHERE id = $1 RETURNING *"#)
            .bind(&game_id)
            .fetch_one(&data.db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "status": "error",
                        "message": "Game not found",
                    })),
                ),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("{:?}", e),
                    })),
                ),
            })?;

    let response = json!({
        "status": "success",
        "message": "Game deleted successfully",
        "data": {
            "deleted_game": query_result
        }
    });

    Ok(Json(response))
}

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json
};
use serde_json::{json, Value};
use crate::models::feedbacks::{AppState, CreateFeedbackInput, Feedback, UpdateFeedbackInput};

// 2. CRUD HANDLERS

// [CREATE] -> POST /api/feedback
pub async fn create_feedback(
    State(state): State<AppState>,
    Json(payload): Json<CreateFeedbackInput>,
) -> Result<(StatusCode, Json<Feedback>), (StatusCode, Json<Value>)> {
    if payload.name.trim().is_empty() || payload.message.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Name and message fields are required" })),
        ));
    }

    // FIXED: Removed the invalid backslashes from $1, $2, etc.
    let result = sqlx::query_as::<_, Feedback>(
        r#"
        INSERT INTO feedback (name, email, phone, message)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, email, phone, message
        "#
    )
    .bind(payload.name)
    .bind(payload.email)
    .bind(payload.phone)
    .bind(payload.message)
    .fetch_one(&state.pool)
    .await;

    match result {
        Ok(feedback) => Ok((StatusCode::CREATED, Json(feedback))),
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if db_err.code() == Some(std::borrow::Cow::Borrowed("23505")) {
                    return Err((
                        StatusCode::CONFLICT,
                        Json(json!({ "error": "A feedback entry with this email already exists" })),
                    ));
                }
            }
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database error: {}", e) })),
            ))
        }
    }
}

// [READ ALL] -> GET /api/feedback
pub async fn get_all_feedback(
    State(state): State<AppState>,
) -> Result<Json<Vec<Feedback>>, (StatusCode, Json<Value>)> {
    let feedbacks = sqlx::query_as::<_, Feedback>("SELECT * FROM feedback ORDER BY id DESC")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    Ok(Json(feedbacks))
}

// [READ SINGLE] -> GET /api/feedback/{id}
pub async fn get_feedback_by_id(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<Feedback>, (StatusCode, Json<Value>)> {
    // FIXED: Removed the invalid backslash from $1
    let result = sqlx::query_as::<_, Feedback>("SELECT * FROM feedback WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    match result {
        Some(feedback) => Ok(Json(feedback)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Feedback item not found" })),
        )),
    }
}

// [UPDATE] -> PUT /api/feedback/{id}
pub async fn update_feedback(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateFeedbackInput>,
) -> Result<Json<Feedback>, (StatusCode, Json<Value>)> {
    if payload.name.trim().is_empty() || payload.message.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Name and message fields are required" })),
        ));
    }

    // FIXED: Removed the invalid backslashes from $1, $2, etc.
    let result = sqlx::query_as::<_, Feedback>(
        r#"
        UPDATE feedback
        SET name = $1, email = $2, phone = $3, message = $4
        WHERE id = $5
        RETURNING id, name, email, phone, message
        "#
    )
    .bind(payload.name)
    .bind(payload.email)
    .bind(payload.phone)
    .bind(payload.message)
    .bind(id)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(updated_feedback)) => Ok(Json(updated_feedback)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Feedback record target missing" })),
        )),
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if db_err.code() == Some(std::borrow::Cow::Borrowed("23505")) {
                    return Err((
                        StatusCode::CONFLICT,
                        Json(json!({ "error": "Email conflict detected with another row" })),
                    ));
                }
            }
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ))
        }
    }
}

// [DELETE] -> DELETE /api/feedback/{id}
pub async fn delete_feedback(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    // FIXED: Removed the invalid backslash from $1
    let rows_affected = sqlx::query("DELETE FROM feedback WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?
        .rows_affected();

    if rows_affected == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "No feedback row matches the target id" })),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}

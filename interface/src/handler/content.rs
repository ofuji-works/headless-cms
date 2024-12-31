use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde::Deserialize;
use serde_json::Value;

use application::usecase::content::{ContentUsecase, CreateContentInput, UpdateContentInput};
use domain::model::content::Content;
use registry::AppRegistry;

use crate::handler::error::{AppError, AppResult};

#[derive(Deserialize)]
pub struct GetContentQuery {}

pub async fn get_contents(
    State(registry): State<AppRegistry>,
    Query(_): Query<GetContentQuery>,
) -> AppResult<Json<Vec<Content>>> {
    let usecase = ContentUsecase::new(registry.content_repository());
    let result = usecase.get().await;

    if let Ok(value) = result {
        return Ok(Json(value));
    }

    Err(AppError::EntityNotFound("".into()))
}

type CreateContentJson = CreateContentInput;

pub async fn create_content(
    State(registry): State<AppRegistry>,
    Json(json): Json<CreateContentJson>,
) -> AppResult<()> {
    let usecase = ContentUsecase::new(registry.content_repository());
    let result = usecase.create(json).await;

    if result.is_ok() {
        return Ok(());
    }

    Err(AppError::CreateRecordError)
}

#[derive(Deserialize)]
pub struct UpdateContentJson {
    pub content_model_id: String,
    pub field_values: Option<Value>,
    pub is_draft: Option<bool>,
}

pub async fn update_content(
    State(registry): State<AppRegistry>,
    Path(id): Path<String>,
    Json(json): Json<UpdateContentJson>,
) -> AppResult<()> {
    let usecase = ContentUsecase::new(registry.content_repository());

    let UpdateContentJson {
        content_model_id,
        field_values,
        is_draft,
    } = json;

    let input = UpdateContentInput::new(id, content_model_id, field_values, is_draft);
    let result = usecase.update(input).await;

    if result.is_ok() {
        return Ok(());
    }

    Err(AppError::UpdateRecordError)
}

pub async fn delete_content(
    State(registry): State<AppRegistry>,
    Path(id): Path<String>,
) -> AppResult<()> {
    let usecase = ContentUsecase::new(registry.content_repository());
    let result = usecase.delete(id).await;

    if result.is_ok() {
        return Ok(());
    }

    Err(AppError::DeleteRecordError)
}

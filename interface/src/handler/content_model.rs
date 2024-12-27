use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde::Deserialize;
use serde_json::Value;

use application::usecase::content_model::{
    ContentModelUsecase, CreateContentModelInput, UpdateContentModelInput,
};
use domain::model::content_model::ContentModel;
use registry::AppRegistry;

use crate::handler::error::{AppError, AppResult};

#[derive(Deserialize)]
pub struct GetContentModelRequest {}

pub async fn get_content_models(
    State(registry): State<AppRegistry>,
    Query(_): Query<GetContentModelRequest>,
) -> AppResult<Json<Vec<ContentModel>>> {
    let usecase = ContentModelUsecase::new(registry.content_model_repository());
    let result = usecase.get().await;

    if let Ok(value) = result {
        return Ok(Json(value));
    }

    Err(AppError::EntityNotFound("".into()))
}

type CreateContentModelJson = CreateContentModelInput;

pub async fn create_content_model(
    State(registry): State<AppRegistry>,
    Json(content_model): Json<CreateContentModelJson>,
) -> AppResult<()> {
    let usecase = ContentModelUsecase::new(registry.content_model_repository());
    let result = usecase.create(content_model).await;

    if result.is_ok() {
        return Ok(());
    }

    Err(AppError::CreateRecordError)
}

#[derive(Deserialize)]
pub struct UpdateContentModelJson {
    pub name: Option<String>,
    pub api_identifier: Option<String>,
    pub description: Option<String>,
    pub fields: Option<Value>,
}

pub async fn update_content_model(
    State(registry): State<AppRegistry>,
    Path(id): Path<String>,
    Json(content_model): Json<UpdateContentModelJson>,
) -> AppResult<()> {
    let usecase = ContentModelUsecase::new(registry.content_model_repository());

    let UpdateContentModelJson {
        name,
        api_identifier,
        description,
        fields,
    } = content_model;

    let input = UpdateContentModelInput::new(id, name, api_identifier, description, fields);
    let result = usecase.update(input).await;

    if result.is_ok() {
        return Ok(());
    }

    Err(AppError::UpdateRecordError)
}

pub async fn delete_content_model(
    State(registry): State<AppRegistry>,
    Path(id): Path<String>,
) -> AppResult<()> {
    let usecase = ContentModelUsecase::new(registry.content_model_repository());
    let result = usecase.delete(id).await;

    if result.is_ok() {
        return Ok(());
    }

    Err(AppError::DeleteRecordError)
}

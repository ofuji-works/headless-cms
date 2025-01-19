use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde::Deserialize;

use application::usecase::content_model::{
    ContentModelUsecase, CreateContentModelInput, UpdateContentModelInput,
};
use domain::model::content_model::ContentModel;
use registry::AppRegistry;
use utoipa::{IntoParams, ToSchema};

use crate::handler::error::{AppError, AppResult};

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct GetContentModelQuery {
    pub limit: usize,
}

#[utoipa::path(
    get,
    path = "/content_models",
    params(GetContentModelQuery), 
    responses(
        (status = 200, description = "Get content model success", body = [ContentModel])
    ),
    tag = "content_models",
)]
pub async fn get_content_models(
    State(registry): State<AppRegistry>,
    Query(query): Query<GetContentModelQuery>,
) -> AppResult<Json<Vec<ContentModel>>> {
    let GetContentModelQuery { limit: _ } = query;
    let usecase = ContentModelUsecase::new(registry.content_model_repository());
    let result = usecase.get().await;

    if let Ok(value) = result {
        return Ok(Json(value));
    }

    Err(AppError::EntityNotFound("".into()))
}

pub type CreateContentModelJson = CreateContentModelInput;

#[utoipa::path(
    post,
    path = "/content_models",
    request_body = CreateContentModelJson,
    responses(
        (status = 200, description = "Create content model success"),
    ),
    tag = "content_models",
)]
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

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct UpdateContentModelJson {
    pub name: Option<String>,
    pub api_identifier: Option<String>,
    pub description: Option<String>,
}

#[utoipa::path(
    put,
    path = "/content_models/{id}",
    params(
        ("id" = String, Path, description = "Content model ID"),
    ),
    request_body = UpdateContentModelJson,
    responses(
        (status = 200, description = "Update content model success"),
    ),
    tag = "content_models",
)]
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
    } = content_model;

    let input = UpdateContentModelInput::new(id, name, api_identifier, description);
    let result = usecase.update(input).await;

    if result.is_ok() {
        return Ok(());
    }

    Err(AppError::UpdateRecordError)
}

#[utoipa::path(
    delete,
    path = "/content_models/{id}",
    params(
        ("id" = String, Path, description = "Content model ID")
    ),
    responses(
        (status = 200, description = "Delete content model success"),
    ),
    tag = "content_models",
)]
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

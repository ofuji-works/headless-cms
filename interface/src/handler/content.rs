use axum::{
    extract::{Path, Query, State},
    response::Json,
};

use application::usecase::content::{ContentUsecase, CreateContentInput, UpdateContentInput};
use domain::model::content::{Content, ContentStatus};
use registry::AppRegistry;

use crate::handler::error::{AppError, AppResult};

#[derive(serde::Deserialize, utoipa::IntoParams, utoipa::ToSchema)]
pub struct GetContentQuery {
    pub limit: usize,
}

#[utoipa::path(
    get,
    path = "/contents",
    params(GetContentQuery),
    responses((status = 200, description = "Get content success")),
    tag = "contents"
)]
pub async fn get_contents(
    State(registry): State<AppRegistry>,
    Query(query): Query<GetContentQuery>,
) -> AppResult<Json<Vec<Content>>> {
    let GetContentQuery { limit: _ } = query;

    let usecase = ContentUsecase::new(registry.content_repository());
    let result = usecase.get().await;

    if let Ok(value) = result {
        return Ok(Json(value));
    }

    Err(AppError::EntityNotFound("".into()))
}

type CreateContentJson = CreateContentInput;

#[utoipa::path(
    post,
    path = "/contents",
    request_body = CreateContentJson,
    responses(
        (status = 200, description = "Create content success")
    ),
    tag = "contents",
)]
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

#[derive(serde::Deserialize, utoipa::IntoParams, utoipa::ToSchema)]
pub struct UpdateContentJson {
    pub content_model_id: Option<String>,
    pub fields: Option<serde_json::Value>,
    pub status: Option<ContentStatus>,
}

#[utoipa::path(
    put,
    path = "/contents/{id}",
    params(
        ("id" = String, Path, description = "contents ID")
    ),
    request_body = UpdateContentJson,
    responses(
        (status = 200, description = "Update content success"),
    ),
    tag = "contents",
)]
pub async fn update_content(
    State(registry): State<AppRegistry>,
    Path(id): Path<String>,
    Json(json): Json<UpdateContentJson>,
) -> AppResult<()> {
    let usecase = ContentUsecase::new(registry.content_repository());

    let UpdateContentJson {
        content_model_id,
        fields,
        status,
    } = json;

    let input = UpdateContentInput::new(id, content_model_id, fields, status);
    let result = usecase.update(input).await;

    if result.is_ok() {
        return Ok(());
    }

    Err(AppError::UpdateRecordError)
}

#[utoipa::path(
    delete,
    path = "/contents/{id}",
    params(
        ("id" = String,  Path, description = "Content ID"),
    ),
    responses(
        (status = 200, description = "Delete content success")
    ),
    tag = "contents",
)]
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

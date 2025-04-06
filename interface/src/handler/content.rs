use axum::{
    extract::{Path, Query, State},
    response::Json,
};

use application::usecase::content::{
    ContentUsecase, CreateContentInput, GetContentInput, UpdateContentInput,
};
use domain::model::content::{Content, ContentStatus};
use registry::AppRegistry;

use crate::handler::error::{AppError, AppResult};

#[derive(serde::Deserialize, utoipa::IntoParams, utoipa::ToSchema)]
pub struct GetContentRequest {
    #[param(example = 0)]
    pub offset: i32,
    #[param(example = 100)]
    pub limit: i32,
    pub keyword: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl From<GetContentRequest> for GetContentInput {
    fn from(value: GetContentRequest) -> Self {
        let GetContentRequest { limit, offset, .. } = value;

        Self { limit, offset }
    }
}

#[utoipa::path(
    get,
    path = "/contents",
    params(GetContentRequest),
    responses((status = 200, description = "Get content success", body = [Content])),
    tag = "contents"
)]
pub async fn get_contents(
    State(registry): State<AppRegistry>,
    Query(query): Query<GetContentRequest>,
) -> AppResult<Json<Vec<Content>>> {
    let usecase = ContentUsecase::new(registry.content_repository());
    let input = GetContentInput::from(query);
    let result = usecase.get(input).await;

    if let Ok(value) = result {
        return Ok(Json(value));
    }

    Err(AppError::EntityNotFound("".into()))
}

#[derive(serde::Deserialize, utoipa::IntoParams, utoipa::ToSchema)]
pub struct CreateContentJson {
    pub title: String,
    pub fields: serde_json::Value,
    pub tag_ids: Vec<String>,
    pub status: ContentStatus,
    pub category_id: String,
}

impl From<CreateContentJson> for CreateContentInput {
    fn from(json: CreateContentJson) -> Self {
        let CreateContentJson {
            title,
            fields,
            tag_ids,
            status,
            category_id,
        } = json;
        let created_by_id: String = "id".into();
        let updated_by_id: String = "id".into();

        Self {
            title,
            fields,
            tag_ids,
            status,
            category_id,
            created_by_id,
            updated_by_id,
        }
    }
}

#[utoipa::path(
    post,
    path = "/contents",
    request_body = CreateContentJson,
    responses(
        (status = 200, description = "Create content success", body = Content)
    ),
    tag = "contents",
)]
pub async fn create_content(
    State(registry): State<AppRegistry>,
    Json(json): Json<CreateContentJson>,
) -> AppResult<()> {
    let input = CreateContentInput::from(json);
    let usecase = ContentUsecase::new(registry.content_repository());
    let result = usecase.create(input).await;

    if result.is_ok() {
        return Ok(());
    }

    Err(AppError::CreateRecordError)
}

#[derive(serde::Deserialize, utoipa::IntoParams, utoipa::ToSchema)]
pub struct UpdateContentJson {
    pub fields: Option<serde_json::Value>,
    pub status: Option<ContentStatus>,
    pub category_id: Option<String>,
}

impl From<UpdateContentJson> for UpdateContentInput {
    fn from(value: UpdateContentJson) -> Self {
        let UpdateContentJson {
            fields,
            status,
            category_id,
        } = value;

        Self {}
    }
}

#[utoipa::path(
    put,
    path = "/contents/{id}",
    params(
        ("id" = String, Path, description = "contents ID")
    ),
    request_body = UpdateContentJson,
    responses(
        (status = 200, description = "Update content success", body = Content),
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
        fields,
        status,
        category_id,
    } = json;

    let mock_id: String = "id".into();
    let input = UpdateContentInput::new(id, category_id, fields, status, mock_id);
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

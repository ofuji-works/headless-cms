use axum::{
    extract::{Path, Query, State},
    response::Json,
};

use application::usecase::category::{
    CategoryUsecase, CreateCategoryInput, GetCategoryInput, UpdateCategoryInput
};
use domain::model::category::Category;
use registry::AppRegistry;

use crate::handler::error::{AppError, AppResult};

#[derive(serde::Deserialize, utoipa::IntoParams, utoipa::ToSchema)]
pub struct GetCategoryQuery {
    #[param(example = 0)]
    pub offset: i32,
    #[param(example = 100)]
    pub limit: i32,
    pub keyword: Option<String>,
}

#[utoipa::path(
    get,
    path = "/categories",
    params(GetCategoryQuery), 
    responses(
        (status = 200, description = "Get category success", body = [Category])
    ),
    tag = "categories",
)]
pub async fn get_categories(
    State(registry): State<AppRegistry>,
    Query(query): Query<GetCategoryQuery>,
) -> AppResult<Json<Vec<Category>>> {
    let GetCategoryQuery { offset, limit, .. } = query;
    let usecase = CategoryUsecase::new(registry.category_repository());
    let input = GetCategoryInput::new(limit, offset);
    let result = usecase.get(input).await;

    if let Ok(value) = result {
        return Ok(Json(value));
    }

    Err(AppError::EntityNotFound("".into()))
}


#[derive(serde::Deserialize, utoipa::IntoParams, utoipa::ToSchema)]
pub struct CreateCategoryJson {
    name: String,
    api_identifier: String,
    description: Option<String>
}

impl From<CreateCategoryJson> for CreateCategoryInput {
    fn from(json: CreateCategoryJson) -> Self {
        let CreateCategoryJson {name, api_identifier, description} = json;

        Self {
            name,
            api_identifier,
            description,
        }
    }
}

#[utoipa::path(
    post,
    path = "/categories",
    request_body = CreateCategoryJson,
    responses(
        (status = 200, description = "Create category success", body = Category),
    ),
    tag = "categories",
)]
pub async fn create_category(
    State(registry): State<AppRegistry>,
    Json(json): Json<CreateCategoryJson>,
) -> AppResult<()> {
    let input = CreateCategoryInput::from(json);
    let usecase = CategoryUsecase::new(registry.category_repository());
    let result = usecase.create(input).await;

    if result.is_ok() {
        return Ok(());
    }

    Err(AppError::CreateRecordError)
}

#[derive(serde::Deserialize, utoipa::IntoParams, utoipa::ToSchema)]
pub struct UpdateCategoryJson {
    pub name: Option<String>,
    pub api_identifier: Option<String>,
    pub description: Option<String>,
}

#[utoipa::path(
    put,
    path = "/categories/{id}",
    params(
        ("id" = String, Path, description = "Category ID"),
    ),
    request_body = UpdateCategoryJson,
    responses(
        (status = 200, description = "Update category success", body = Category),
    ),
    tag = "categories",
)]
pub async fn update_category(
    State(registry): State<AppRegistry>,
    Path(id): Path<String>,
    Json(category): Json<UpdateCategoryJson>,
) -> AppResult<()> {
    let usecase = CategoryUsecase::new(registry.category_repository());

    let UpdateCategoryJson {
        name,
        api_identifier,
        description,
    } = category;

    let input = UpdateCategoryInput::new(id, name, api_identifier, description);
    let result = usecase.update(input).await;

    if result.is_ok() {
        return Ok(());
    }

    Err(AppError::UpdateRecordError)
}

#[utoipa::path(
    delete,
    path = "/categories/{id}",
    params(
        ("id" = String, Path, description = "Category ID")
    ),
    responses(
        (status = 200, description = "Delete category success"),
    ),
    tag = "categories",
)]
pub async fn delete_category(
    State(registry): State<AppRegistry>,
    Path(id): Path<String>,
) -> AppResult<()> {
    let usecase = CategoryUsecase::new(registry.category_repository());
    let result = usecase.delete(id).await;

    if result.is_ok() {
        return Ok(());
    }

    Err(AppError::DeleteRecordError)
}

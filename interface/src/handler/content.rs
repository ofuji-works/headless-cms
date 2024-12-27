use application::usecase::content::{CreateContentInput, UpdateContentInput};
use axum::{
    extract::{Path, Query, State},
    Json,
};

use registry::AppRegistry;

use crate::handler::error::AppResult;

pub struct GetContentQuery {}

pub async fn get_contents(
    State(registry): State<AppRegistry>,
    Query(query): Query<GetContentQuery>,
) -> AppResult<()> {
    Ok(())
}

type CreateContentJson = CreateContentInput;

pub async fn create_content(
    State(registry): State<AppRegistry>,
    Json(json): Json<CreateContentJson>,
) -> AppResult<()> {
    Ok(())
}

type UpdateContentJson = UpdateContentInput;

pub async fn update_content(
    State(registry): State<AppRegistry>,
    Path(id): Path<String>,
    Json(json): Json<UpdateContentJson>,
) -> AppResult<()> {
    Ok(())
}

pub async fn delete_content(
    State(registry): State<AppRegistry>,
    Path(id): Path<String>,
) -> AppResult<()> {
    Ok(())
}

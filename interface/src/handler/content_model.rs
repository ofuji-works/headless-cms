use axum::{
    extract::{
        State,
        Query,
    },
    response::Json
};
use serde::Deserialize;

use registry::AppRegistry;
use application::usecase::content_model::ContentModelUsecase;
use domain::model::content_model::ContentModel;

use crate::handler::error::{
    AppError,
    AppResult,
};


#[derive(Deserialize)]
pub struct GetContentModelRequest {}

pub async fn get_content_models(State(registry): State<AppRegistry>, Query(_): Query<GetContentModelRequest>) -> AppResult<Json<Vec<ContentModel>>> {
    let usecase = ContentModelUsecase::new(registry.content_model_repository());
    let result = usecase.get().await;

    if let Ok(value) = result {
        return Ok(Json(value));
    }

    Err(AppError::EntityNotFound("".into()))
} 


use axum::{routing::get, Router};
use registry::AppRegistry;

use crate::handler::content_model::get_content_models;

pub fn build_content_model_routers() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/", get(get_content_models));

    Router::new().nest("/content_models", routers)
}


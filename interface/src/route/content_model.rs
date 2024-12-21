use axum::{
    routing,
    Router
};
use registry::AppRegistry;

use crate::handler::content_model::{
    create_content_model, delete_content_model, get_content_models, update_content_model
};

pub fn build_content_model_routers() -> Router<AppRegistry> {
    let routers = Router::new()
        .route(
            "/",
            routing::get(get_content_models)
                .post(create_content_model)
        )
        .route(
            "/:id",
            routing::put(update_content_model)
                .delete(delete_content_model)
        );

    Router::new().nest("/content_models", routers)
}


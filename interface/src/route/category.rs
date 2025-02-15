use axum::{routing, Router};

use registry::AppRegistry;

use crate::handler::category::{create_category, delete_category, get_categories, update_category};

pub fn build_category_routers() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/", routing::get(get_categories).post(create_category))
        .route(
            "/:id",
            routing::put(update_category).delete(delete_category),
        );

    Router::new().nest("/categories", routers)
}

use axum::{routing, Router};
use registry::AppRegistry;

use crate::handler::content::{create_content, delete_content, get_contents, update_content};

pub fn build_contents_routers() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/", routing::get(get_contents).post(create_content))
        .route("/:id", routing::put(update_content).delete(delete_content));

    Router::new().nest("/contents", routers)
}

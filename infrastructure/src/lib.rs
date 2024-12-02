use axum::{
    routing::get,
    Router,
};

pub struct WebApp;

impl WebApp {
    pub async fn bootstrap() {
        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route("/health", get(|| async { "healthy" }));
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

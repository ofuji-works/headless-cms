pub mod handler;
pub mod route;

use std::net::{Ipv4Addr, SocketAddr};

use anyhow::{Error, Result};
use axum::{serve, Router};
use registry::AppRegistry;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::route::content::build_contents_routers;
use crate::route::content_model::build_content_model_routers;
use crate::route::health::build_health_check_routers;
use crate::route::swagger::ApiDoc;

pub struct WebApp;

impl WebApp {
    pub async fn run(registry: AppRegistry) -> Result<()> {
        let app = Router::new()
            .merge(build_health_check_routers())
            .merge(build_contents_routers())
            .merge(build_content_model_routers())
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .with_state(registry);

        let port = std::env::var("APP_PORT")?.parse()?;
        let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);
        let listener = TcpListener::bind(&addr).await?;

        serve(listener, app).await.map_err(Error::from)
    }
}

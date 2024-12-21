pub mod handler;
pub mod route;

use std::net::{Ipv4Addr, SocketAddr};

use axum::{Router, serve};
use anyhow::{Error, Result};
use registry::AppRegistry;
use tokio::net::TcpListener;

use crate::route::health::build_health_check_routers;
use crate::route::content_model::build_content_model_routers;

pub struct WebApp;

impl WebApp {
    pub async fn run(registry: AppRegistry) -> Result<()> {
        let app = Router::new()
            .merge(build_health_check_routers())
            .merge(build_content_model_routers())
            .with_state(registry);

        let port = std::env::var("APP_PORT")?.parse()?;
        let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);
        let listener = TcpListener::bind(&addr).await?;

        serve(listener, app).await.map_err(Error::from)
    }
}


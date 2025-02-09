use anyhow::Result;
use interface::WebApp;
use registry::AppRegistry;
use shared::config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::new()?;
    let registry = AppRegistry::new(config);
    WebApp::run(registry).await
}

use anyhow::Result;
use interface::WebApp;
use infrastructure::database::connect_database_with;
use shared::config::AppConfig;
use registry::AppRegistry;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::new()?;
    let conn = connect_database_with(config.database);
    let registry = AppRegistry::new(conn);
    WebApp::run(registry).await
}

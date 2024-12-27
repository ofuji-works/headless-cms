use anyhow::Result;
use infrastructure::database::connect_database_with;
use interface::WebApp;
use registry::AppRegistry;
use shared::config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::new()?;
    let conn = connect_database_with(config.database);
    let registry = AppRegistry::new(conn);
    WebApp::run(registry).await
}

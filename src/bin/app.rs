use infrastructure::WebApp;

#[tokio::main]
async fn main () {
    WebApp::bootstrap().await;
}

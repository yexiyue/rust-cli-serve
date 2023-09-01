use tracing::info;
use tracing_subscriber;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let token=rust_server::jwt::Claims::new("yexiyue22".to_string(), rust_server::user::server::Role::Admin.to_string());
    let token=token.encode().unwrap();
    info!("Token: {}", token);
    rust_server::start().await?;
    Ok(())
}

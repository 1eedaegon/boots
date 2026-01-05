use {{project_name_snake}}_api::create_router;
use std::net::SocketAddr;
use tracing::info;

pub async fn run(port: u16) -> anyhow::Result<()> {
    let app = create_router();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("Server running on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

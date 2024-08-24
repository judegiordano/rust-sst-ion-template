use lambda_http::Error;
use rust_sst_ion_template::controllers::routes;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    let app = axum::Router::new().nest("/api", routes());
    // bind to localhost when running cargo dev
    if cfg!(debug_assertions) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        tracing::info!("listening on {:?}", listener.local_addr()?);
        return Ok(axum::serve(listener, app).await?);
    }
    lambda_http::run(app).await
}

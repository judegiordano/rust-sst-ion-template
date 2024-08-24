mod dev;

pub fn routes() -> axum::Router {
    axum::Router::new().nest("/dev", dev::router())
}

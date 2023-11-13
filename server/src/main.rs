use axum::handler::HandlerWithoutStateExt;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_pages()));

    axum::Server::bind("0.0.0.0:3000".parse().unwrap())
        .serve(app.into_service());
}

async fn get_pages() -> &'static str {
    ""
}
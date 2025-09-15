use axum::{
    extract::Path,
    response::Html,
    routing::get,
    Router,
};
use simple_logger::SimpleLogger;
use std::fs::read_to_string;

async fn handler() -> Html<String> {
    log::info!("Serving index");
    Html(read_to_string("./pages/index.html").unwrap())
}

async fn wild(Path(path): Path<String>) -> Html<String> {
    log::info!("Serving Wildcard {path}");
    Html(format!("Coming from {path}"))
}

async fn reviews(Path(path): Path<String>) -> Html<String> {
    log::info!("Serving Review {path}");
    Html(read_to_string(format!("./pages/reviews/{path}.html")).unwrap_or(format!("Review could not be found")))
}

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();
    log::info!("Starting up webby server");

    let app = Router::new().route("/", get(handler)).route("/review/{*key}", get(reviews)).route("/{*key}", get(wild));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

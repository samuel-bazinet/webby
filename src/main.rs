use axum::{Router, extract::Path, response::Html, routing::get};
use simple_logger::SimpleLogger;
use std::fs::read_to_string;

const CONTENT_PATH: &str = "./pages/webby_f/build/";

async fn index() -> Html<String> {
    log::info!("Serving index");
    Html(read_to_string(format!("{CONTENT_PATH}index.html")).unwrap())
}

async fn review() -> Html<String> {
    log::info!("Serving Review root page");
    Html(read_to_string(format!("{CONTENT_PATH}review.html")).unwrap())
}

async fn reviews(Path(path): Path<String>) -> Html<String> {
    log::info!("Serving Review {path}");
    Html(
        read_to_string(format!("{CONTENT_PATH}review/{path}.html"))
            .unwrap_or(format!("Review could not be found")),
    )
}

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();
    log::info!("Starting up webby server");

    let app = Router::new()
        .route("/", get(index))
        .route("/review", get(review))
        .route("/review/{*key}", get(reviews));

    let listener = if cfg!(feature = "dev") {
        tokio::net::TcpListener::bind("127.0.0.1:8080")
            .await
            .unwrap()
    } else {
        tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap()
    };

    axum::serve(listener, app).await.unwrap();
}

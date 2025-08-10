use axum::{Router, response::Response, routing::get};

use email_examples::{email::github::Email, render};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    // run with hyper, listening globally
    println!("listening on http://0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Response {
    let output = render::<Email>().await;

    Response::builder()
        .header("content-type", "text/html")
        .body(output.into())
        .expect("valid response")
}

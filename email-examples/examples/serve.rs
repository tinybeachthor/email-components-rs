use axum::{Router, response::Response, routing::get};
use yew::ServerRenderer;

use email_examples::email::github::Email;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(render));

    // run with hyper, listening globally
    println!("listening on http://0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn render() -> Response {
    let renderer = ServerRenderer::<Email>::new();
    let output = renderer.render().await;

    Response::builder()
        .header("content-type", "text/html")
        .body(output.into())
        .expect("valid response")
}

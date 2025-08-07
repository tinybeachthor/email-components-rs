use axum::{Router, routing::get};
use yew::ServerRenderer;

use email_examples::email::github::Email;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(render));

    // run with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn render() -> String {
    let renderer = ServerRenderer::<Email>::new();
    renderer.render().await
}

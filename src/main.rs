use yew::prelude::*;
use yew::ServerRenderer;

#[function_component]
fn App() -> Html {
    html! {<div>{"Hello, World!"}</div>}
}

#[tokio::main]
async fn main() {
    let renderer = ServerRenderer::<App>::new();

    let rendered = renderer.render().await;

    println!("{}", rendered);
}

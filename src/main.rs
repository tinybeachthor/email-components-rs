use yew::prelude::*;
use yew::ServerRenderer;

use email_components_rs::components::EmailHtml;

#[function_component]
fn App() -> Html {
    html! {
        <EmailHtml>
            <div>{"Hello, World!"}</div>
        </EmailHtml>
    }
}

#[tokio::main]
async fn main() {
    let renderer = ServerRenderer::<App>::new();
    let rendered = renderer.render().await;
    println!("{}", rendered);
}

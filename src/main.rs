use yew::prelude::*;
use yew::ServerRenderer;

use email_components_rs::components::EmailHtml;

const DOCTYPE: &str =
    r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">"#;

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
    let output = format!("{DOCTYPE}{rendered}");
    println!("{}", output);
}

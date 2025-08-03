use yew::ServerRenderer;
use yew::prelude::*;

use email_components_rs::components::{body::Body, email_html::EmailHtml};

const DOCTYPE: &str = r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">"#;

#[function_component]
fn App() -> Html {
    html! {
        <EmailHtml>
            <Body>
                <div>
                    {"Hello, World!"}
                </div>
            </Body>
        </EmailHtml>
    }
}

#[tokio::main]
async fn main() {
    let renderer = ServerRenderer::<App>::new();
    let rendered = renderer.render().await;

    let opts = tidier::FormatOptions::new().tabs(true).strip_comments(true);
    let output = tidier::format(rendered, false, &opts).expect("format html");

    println!("{DOCTYPE}\n{output}");
}

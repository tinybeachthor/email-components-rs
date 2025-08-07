use yew::ServerRenderer;

use email_examples::email::github::Email;

const DOCTYPE: &str = r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">"#;

#[tokio::main]
async fn main() {
    let renderer = ServerRenderer::<Email>::new();
    let rendered = renderer.render().await;

    let opts = tidier::FormatOptions::new().tabs(true).strip_comments(true);
    let output = tidier::format(rendered, false, &opts).expect("format html");

    println!("{DOCTYPE}\n{output}");
}

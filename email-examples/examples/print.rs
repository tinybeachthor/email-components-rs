use email_examples::{email::github::Email, render};

#[tokio::main]
async fn main() {
    let rendered = render::<Email>().await;

    let opts = tidier::FormatOptions::new().tabs(true).strip_comments(true);
    let output = tidier::format(rendered, false, &opts).expect("format html");

    println!("{output}");
}

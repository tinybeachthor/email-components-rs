use yew::ServerRenderer;
use yew::prelude::*;

use email_components_rs::components::{Body, EmailHtml, Container, Text, Link, Img};

const DOCTYPE: &str = r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">"#;

#[function_component]
fn App() -> Html {
    // TODO
    let base_url = "";
    let username = "USERNAME";

    html! {
        <EmailHtml>
        <Body>
        <Container>
            <Img
              src={format!("{base_url}/static/github.png")}
              width="32"
              height="32"
              alt="Github"
            />

            <Text>
              <strong>{'@'}{username}</strong>
              {", a personal access was created on your account."}
            </Text>

            <Section>
              <Text>
                {"Hey "}<strong>{username}</strong>{'!'}
              </Text>
              <Text>
                {"A fine-grained personal access token ("}
                <Link>{"resend"}</Link>
                {") was recently added to your account."}
              </Text>

              <Button>{"View your token"}</Button>
            </Section>

            <Text>
              <Link>{"Your security audit log"}</Link>
              {" - "}
              <Link>{"Contact support"}</Link>
            </Text>

            <Text>
              {"GitHub, Inc. - 88 Colin P Kelly Jr Street - San Francisco, CA 94107"}
            </Text>
        </Container>
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

use email_components::{Button, Container, EmailHtml, Img, Link, Section, Text};
use stylist::yew::use_style;
use yew::{Html, function_component, html};

#[function_component]
pub fn Email() -> Html {
    // TODO
    let base_url = "";
    let username = "USERNAME";

    let main = use_style!(
        "
        max-width: 480px;
        margin: 0 auto;
        padding: 20px 0 48px;
        font-family: -apple-system,BlinkMacSystemFont,\"Segoe UI\",Helvetica,Arial,sans-serif,\"Apple Color Emoji\",\"Segoe UI Emoji\";
    "
    );
    let body = use_style!(
        "
        padding: 24px;
        border: solid 1px #dedede;
        border-radius: 5px;
        text-align: center;
    "
    );
    let text = use_style!(
        "
        margin: 0 0 10px 0;
        text-align: left;
    "
    );
    let button = use_style!(
        "
        font-size: 14px;
        background-color: #28a745;
        color: #fff;
        line-height: 1.5;
        border-radius: 0.5em;
        padding: 12px 24px;
    "
    );
    let links = use_style!(
        "
        text-align: center;
    "
    );
    let footer = use_style!(
        "
        color: #6a737d;
        text-align: center;
        margin-top: 60px;
    "
    );

    html! {
        <EmailHtml>
            <Container class={main}>
                // Title
                //
                <Img
                    src={format!("{base_url}/static/github.png")}
                    width="32"
                    height="32"
                    alt="Github"
                />
                <Text>
                    <strong>{ '@' }{ username }</strong>
                    { ", a personal access token was created on your account." }
                </Text>
                // Body
                //
                <Section class={body}>
                    <Text class={text.clone()}>
                        { "Hey " }
                        <strong>{ username }</strong>
                        { '!' }
                    </Text>
                    <Text class={text}>
                        { "A fine-grained personal access token (" }
                        <Link>{ "token" }</Link>
                        { ") was recently added to your account." }
                    </Text>
                    <Button class={button}>{ "View your token" }</Button>
                </Section>
                <Text class={links}>
                    <Link>{ "Your security audit log" }</Link>
                    { " - " }
                    <Link>{ "Contact support" }</Link>
                </Text>
                // Footer
                //
                <Text class={footer}>
                    { "GitHub, Inc. - 88 Colin P Kelly Jr Street - San Francisco, CA 94107" }
                </Text>
            </Container>
        </EmailHtml>
    }
}

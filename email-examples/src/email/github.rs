use email_components::{Button, Container, EmailHtml, Img, Link, Section, Text};
use stylist::yew::use_style;
use yew::{function_component, html, Html};

#[function_component]
pub fn Email() -> Html {
    // TODO
    let base_url = "";
    let username = "USERNAME";

    let container = use_style!("
        max-width: '480px';
        margin: '0 auto';
        padding: '20px 0 48px';
    ");
    let footer = use_style!(
        "color: '#6a737d';
        text-align: 'center';
        margin-top: '60px';
    ");

    html! {
        <EmailHtml>
                <Container class={container}>
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
                        { ", a personal access was created on your account." }
                    </Text>
                    // Body
                    //
                    <Section>
                        <Text>
                            { "Hey " }
                            <strong>{ username }</strong>
                            { '!' }
                        </Text>
                        <Text>
                            { "A fine-grained personal access token (" }
                            <Link>{ "resend" }</Link>
                            { ") was recently added to your account." }
                        </Text>
                        <Button>{ "View your token" }</Button>
                    </Section>
                    <Text>
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

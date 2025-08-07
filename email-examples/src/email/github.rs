use email_components::{
    Body, Button, Container, EmailHtml, Img, Link, Section, Text,
};
use yew::{function_component, html, Html};

#[function_component]
pub fn Email() -> Html {
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
                        <strong>{ '@' }{ username }</strong>
                        { ", a personal access was created on your account." }
                    </Text>
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
                    <Text>
                        { "GitHub, Inc. - 88 Colin P Kelly Jr Street - San Francisco, CA 94107" }
                    </Text>
                </Container>
            </Body>
        </EmailHtml>
    }
}

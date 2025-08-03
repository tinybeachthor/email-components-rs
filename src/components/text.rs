use yew::{Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component]
pub fn Text(props: &Props) -> Html {
    let Props { children } = props;

    html! {
        <p
            style="fontSize:'14px'; lineHeight:'24px'"
        >
            { children.clone() }
        </p>
    }
}

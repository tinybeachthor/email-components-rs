use stylist::yew::use_style;
use yew::{Classes, Html, Properties, classes, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    pub children: Html,
}

#[function_component]
pub fn Text(props: &Props) -> Html {
    let Props { class, children } = props;

    let default = use_style!("fontSize:'14px'; lineHeight:'24px';");

    html! {
        <p
            class={classes!(
                default,
                class.clone(),
            )}
        >
            { children.clone() }
        </p>
    }
}

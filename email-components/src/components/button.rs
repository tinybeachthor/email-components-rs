use stylist::yew::use_style;
use yew::{AttrValue, Classes, Html, Properties, classes, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::Static("_blank"))]
    pub target: AttrValue,

    #[prop_or_default]
    pub class: Classes,
    pub children: Html,
}

#[function_component]
pub fn Button(props: &Props) -> Html {
    let Props {
        target,
        class,
        children,
    } = props;

    let default = use_style!(
        "
        line-height: 100%;
        text-decoration: none;
        display: inline-block;
        max-width: 100%;
        mso-padding-alt: 0px;
    "
    );

    html! {
        <a
            target={target}
            class={classes!(
                default,
                class.clone(),
            )}
        >
            <span
                style="
                    max-width: 100%;
                    display: inline-block;
                    line-height: 120%;
                    mso-padding-alt: 0px
                "
            >
                { children.clone() }
            </span>
        </a>
    }
}

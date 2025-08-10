use yew::{AttrValue, Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::Static("_blank"))]
    pub target: AttrValue,

    pub children: Html,
}

#[function_component]
pub fn Button(props: &Props) -> Html {
    let Props { target, children } = props;

    html! {
        <a
            style="
                line-height: 100%;
                text-decoration: none;
                display: inline-block;
                max-width: 100%;
                mso-padding-alt: 0px
            "
            target={target}
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

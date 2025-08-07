use yew::{AttrValue, Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::Static("_blank"))]
    pub target: AttrValue,

    pub children: Html,
}

#[function_component]
pub fn Link(props: &Props) -> Html {
    let Props { target, children } = props;

    html! {
        <a style="color: '#067df7'; textDecorationLine: 'none'" target={target}>
            { children.clone() }
        </a>
    }
}

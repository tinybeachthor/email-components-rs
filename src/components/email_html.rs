use yew::{function_component, AttrValue, Properties};

#[derive(Properties, PartialEq)]
pub struct EmailHtmlProps {
    #[prop_or(AttrValue::Static("en"))]
    pub lang: AttrValue,
    #[prop_or(AttrValue::Static("ltr"))]
    pub dir: AttrValue,

    pub children: yew::Html,
}

#[function_component]
pub fn EmailHtml(props: &EmailHtmlProps) -> yew::Html {
    let EmailHtmlProps { lang, dir, children } = props;

    yew::html! {
        <html lang={lang} dir={dir}>
            { children.clone() }
        </html>
    }
}

use yew::{AttrValue, Properties, function_component};

pub const HEAD_PLACEHOLDER: &str = "###HEAD_PLACEHOLDER###";

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::Static("en"))]
    pub lang: AttrValue,
    #[prop_or(AttrValue::Static("ltr"))]
    pub dir: AttrValue,

    pub children: yew::Html,
}

#[function_component]
pub fn EmailHtml(props: &Props) -> yew::Html {
    let Props {
        lang,
        dir,
        children,
    } = props;

    yew::html! {
        <html lang={lang} dir={dir}>
            <head>
                <meta charset="utf-8" />
                {HEAD_PLACEHOLDER}
            </head>
            <body>
                { children.clone() }
            </body>
        </html>
    }
}

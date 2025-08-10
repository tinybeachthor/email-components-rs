pub use yew::{AttrValue, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub src: AttrValue,

    #[prop_or(AttrValue::Static("32"))]
    pub width: AttrValue,
    #[prop_or(AttrValue::Static("32"))]
    pub height: AttrValue,

    pub alt: AttrValue,
}

#[function_component]
pub fn Img(props: &Props) -> yew::Html {
    let Props {
        src,
        width,
        height,
        alt,
    } = props;

    yew::html! {
        <img
            src={src}
            width={width}
            height={height}
            alt={alt}
            style="display: block; outline: none; border: none; text-decoration: none"
        />
    }
}

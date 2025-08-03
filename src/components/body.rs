use yew::{function_component, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: yew::Html,
}

#[function_component]
pub fn Body(props: &Props) -> yew::Html {
    let Props { children } = props;

    yew::html! {
        <body>
            { children.clone() }
        </body>
    }
}

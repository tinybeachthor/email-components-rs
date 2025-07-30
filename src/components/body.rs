use yew::{function_component, Properties};

#[derive(Properties, PartialEq)]
pub struct BodyProps {
    pub children: yew::Html,
}

#[function_component]
pub fn Body(props: &BodyProps) -> yew::Html {
    let BodyProps { children } = props;

    yew::html! {
        <body>
            { children.clone() }
        </body>
    }
}

use yew::{Classes, Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub class: Classes,
    pub children: Html,
}

#[function_component]
pub fn Section(props: &Props) -> Html {
    let Props { class, children } = props;

    html! {
        <table
            align="center"
            width="100%"
            border=0
            cellpadding="0"
            cellspacing="0"
            role="presentation"
            class={class.clone()}
        >
            <tbody>
                <tr>
                    <td>{ children.clone() }</td>
                </tr>
            </tbody>
        </table>
    }
}

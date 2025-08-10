use yew::{Classes, Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    pub children: Html,
}

#[function_component]
pub fn Container(props: &Props) -> Html {
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
                <tr style="width: 100%">
                    <td>{ children.clone() }</td>
                </tr>
            </tbody>
        </table>
    }
}

use yew::{Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component]
pub fn Section(props: &Props) -> Html {
    let Props { children } = props;

    html! {
        <table
            align="center"
            width="100%"
            border=0
            cellPadding="0"
            cellSpacing="0"
            role="presentation"
        >
            <tbody>
                <tr>
                    <td>{ children.clone() }</td>
                </tr>
            </tbody>
        </table>
    }
}

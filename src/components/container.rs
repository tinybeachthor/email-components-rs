use yew::{Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component]
pub fn Container(props: &Props) -> Html {
    let Props { children } = props;

    html! {
        <table
            align="center"
            width="100%"
            border={0}
            cellPadding="0"
            cellSpacing="0"
            role="presentation"
            style="maxWidth:'37.5em'"
          >
            <tbody>
                <tr style="width:'100%'">
                    <td>
                        { children.clone() }
                    </td>
                </tr>
            </tbody>
        </table>
    }
}

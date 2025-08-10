use stylist::{manager::{render_static, StyleManager}, yew::ManagerProvider};
use yew::{function_component, html, BaseComponent, Html, Properties, ServerRenderer};

const DOCTYPE: &str = r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">"#;

pub async fn render<C>(props: C::Properties) -> String
where
    C: BaseComponent,
    C::Properties: Send + Clone,
{
    let (writer, reader) = render_static();

    let renderer = ServerRenderer::<Wrapped<C>>::with_props(|| {
        let style_manager = StyleManager::builder()
            .writer(writer)
            .build()
            .expect("failed to create style manager");
        Props { style_manager, inner_props: props }
    });
    let rendered = renderer.render().await;

    let style_data = reader.read_style_data();
    let mut styles = String::new();
    style_data.write_static_markup(&mut styles)
        .expect("failed to read styles from style manager");

    println!("{styles}");

    format!("{DOCTYPE}\n{rendered}")
}

#[derive(Properties, PartialEq)]
struct Props<P>
where
    P: PartialEq,
{
    pub style_manager: StyleManager,
    pub inner_props: P,
}

#[function_component]
fn Wrapped<C>(props: &Props<C::Properties>) -> Html
where
    C: BaseComponent,
    C::Properties: Clone,
{
    let Props { style_manager, inner_props } = props;

    html! {
        <ManagerProvider manager={style_manager.clone()}>
            <C ..inner_props.clone() />
        </ManagerProvider>
    }
}

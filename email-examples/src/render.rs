use yew::{BaseComponent, ServerRenderer};

const DOCTYPE: &str = r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">"#;

pub async fn render<C>() -> String
where
    C: BaseComponent,
    C::Properties: Default,
{
    let renderer = ServerRenderer::<C>::new();
    let rendered = renderer.render().await;

    format!("{DOCTYPE}\n{rendered}")
}

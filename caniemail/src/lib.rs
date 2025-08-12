mod checks;
mod checker;

use html_parser::Dom;

pub use checker::Checker;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to parse html: {0}")]
    ParseHtml(#[source] html_parser::Error),
}

pub async fn check(html: &str) -> Result<(), Error> {
    let dom = Dom::parse(html)
        .map_err(Error::ParseHtml)?;

    let parsing_errors = dom.errors;

    // TODO

    Ok(())
}

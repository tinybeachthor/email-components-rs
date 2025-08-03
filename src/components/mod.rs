#![allow(non_snake_case)]

pub mod body;
pub mod email_html;
pub mod container;
pub mod text;
pub mod link;

pub use body::Body;
pub use email_html::EmailHtml;
pub use container::Container;
pub use text::Text;
pub use link::Link;

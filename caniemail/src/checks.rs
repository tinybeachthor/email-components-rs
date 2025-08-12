use serde::Deserialize;

#[derive(Deserialize)]
pub struct Checks {
    pub data: Vec<Check>,
}

#[derive(Deserialize)]
pub struct Check {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub category: String,

    pub tags: Vec<String>,
    pub keywords: String,
}
impl Check {
    pub fn is_html(&self) -> bool {
        self.category == "html"
    }

    pub fn is_html_element(&self) -> bool {
        if !self.is_html() {
            return false;
        }

        // TODO

        true
    }
}

use crate::checks::{Check, Checks};

const LATEST_CHECKS: &str = include_str!("../data/2025-08-11.json");

pub struct Checker {
    html_element_checks: Vec<Check>,
}

impl Checker {
    pub fn new() -> Self {
        let checks: Checks = serde_json::from_str(LATEST_CHECKS).unwrap();

        let mut html_element_checks = Vec::new();

        for check in checks.data {
            if !check.is_html_element() {
                html_element_checks.push(check);
            }
        }

        Self {
            html_element_checks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_latest_checks() {
        let _checker = Checker::new();
    }
}

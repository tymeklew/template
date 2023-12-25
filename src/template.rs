use regex::{Captures, Regex};

use crate::Context;

const REGEX_STR: &str = r"\{\{(.*?)\}\}";

pub struct Template {
    pub template: String,
}

impl Template {
    pub fn new<S>(temp: S) -> Template
    where
        S: ToString,
    {
        Template {
            template: temp.to_string(),
        }
    }

    pub fn render(&self, context: Context) -> String {
        let reg = Regex::new(REGEX_STR).unwrap();

        reg.replace(&self.template, |caps: &Captures| {
            println!("{:?}", caps);
            String::new()
        })
        .to_string()
    }
}

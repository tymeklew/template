use regex::{Captures, Regex};

use crate::{error::Error, Context};

const REGEX_STR: &str = r"\{\{(.*?)\}\}";

/// The template struct from which stuff is rendered
/// ```
/// use template::prelude::*;
///
/// let ctx = ctx ! {
///     "name" => "kitty"
/// };
/// let template = Template::new("Hello {{name}}");
/// assert_eq!(template.render(ctx).unwrap() , "Hello kitty");
/// ```
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

    pub fn render(&self, context: Context) -> Result<String, Error> {
        let reg = Regex::new(REGEX_STR).unwrap();

        Ok(reg
            .replace(&self.template, |caps: &Captures| {
                // Get the string value
                let key = caps.get(1).unwrap().as_str().to_string();

                let val = match &context.0.get(&key) {
                    Some(val) => val.to_string(),
                    None => String::new(),
                };

                val
            })
            .to_string())
    }
}

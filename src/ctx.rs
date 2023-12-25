use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data {
    String(String),
    Int(i32),
    Bool(bool),
}

/// The context struct for data for templates this is where the stuff will be replaced with that
/// data. Dont't use this instead use the ctx! {} macro to generate the context
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context(pub HashMap<String, Data>);

impl Context {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

/// Returns a instance of Context
/// # Examples
/// ```
/// use template::ctx;
///
/// let context = ctx! {
///     "key" : 10,
/// };
/// ```
#[macro_export]
macro_rules! ctx {
    ($($key:expr => $val:expr),*) => {{
        let  mut context = Context::new();
        $(
            context.0.insert($key.to_string() , $val.into());
        )*
        context
    }};

}

impl From<i32> for Data {
    fn from(value: i32) -> Self {
        Data::Int(value)
    }
}

impl From<String> for Data {
    fn from(value: String) -> Self {
        Data::String(value)
    }
}

impl From<&str> for Data {
    fn from(value: &str) -> Self {
        Data::String(value.to_string())
    }
}

impl From<bool> for Data {
    fn from(value: bool) -> Self {
        Data::Bool(value)
    }
}

impl ToString for Data {
    fn to_string(&self) -> String {
        match self {
            Data::Int(i) => i.to_string(),
            Data::Bool(b) => b.to_string(),
            Data::String(s) => s.to_string(),
        }
    }
}

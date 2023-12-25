#[macro_use]
pub mod ctx;
pub mod template;
pub use ctx::Context;
pub use template::Template;

pub mod prelude {
    pub use crate::ctx;
    pub use crate::ctx::Context;
    pub use crate::template::Template;
}

#[cfg(test)]
mod tests {

    use crate::{
        ctx::{Context, Data},
        template::Template,
    };

    #[test]
    fn context() {
        let mut context = Context::new();
        context.0.insert(String::from("key"), Data::Int(10));

        let other = ctx! {
            "key" => 10
        };

        assert_eq!(context, other);
    }

    #[test]
    fn template() {
        let template = Template::new("Hello {{name}}");

        template.render(ctx! {
            "name" => "meow"
        });
    }
}

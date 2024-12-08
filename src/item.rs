use serde::Serialize;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none] // must go before the serde::Serialize attribute
#[derive(TypedBuilder, Serialize)]
#[builder(
    doc,
    field_defaults(default, setter(into)),
    mutators(
        // This is a custom mutator that allows us to add multiple arguments to the `arg` field.
        fn arg(&mut self, arg: impl Into<String>) -> &mut Self {
            match self.arg {
                Some(ref mut args) => {
                    args.push(arg.into());
                    self
                },
                None => {
                    self.arg = Some(vec![arg.into()]);
                    self
                }
            }
        }
        // end of custom mutator `arg`
    )
)]
pub struct AlfredItem {
    #[builder(setter(
        strip_option,
        doc = "A unique identifier for the item. It allows Alfred to learn about the item for subsequent sorting and ordering of the user's actioned results."
    ))]
    id: Option<String>,
    #[builder(setter(
        doc = "The title displayed in the result row. There are no options for this element and it is essential that this element is populated."
    ))]
    title: String,
    #[builder(setter(strip_option, doc = "The subtitle displayed in the result row.",))]
    subtitle: Option<String>,
    #[builder(
        setter(
            doc = "The argument which is passed through the workflow to the connected output action."
        ),
        via_mutators
    )]
    arg: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_check() {
        let item = AlfredItem::builder()
            .id("42")
            .title("Hello, World!")
            .subtitle("This is a subtitle")
            .arg("arg1")
            .arg("arg2")
            .build();
        insta::assert_json_snapshot!(item);
    }

    #[test]
    fn num_of_arg() {
        let no_arg = AlfredItem::builder().title("Hello, World!").build();
        insta::assert_json_snapshot!(no_arg);
    }
}

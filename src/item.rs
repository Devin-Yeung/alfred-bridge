use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default)]
pub enum Type {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "file:skipcheck")]
    FileSkipCheck,
}

#[derive(Serialize)]
pub enum IconType {
    #[serde(rename = "fileicon")]
    FileIcon,
    #[serde(rename = "filetype")]
    FileType,
}

#[serde_with::skip_serializing_none] // must go before the serde::Serialize attribute
#[derive(Serialize, TypedBuilder)]
#[builder(doc, field_defaults(default, setter(into)))]
pub struct Icon {
    #[builder(setter(strip_option, doc = "The type of icon to display."))]
    r#type: Option<IconType>,
    #[builder(setter(doc = "The path to the icon."))]
    path: String,
}

#[serde_with::skip_serializing_none] // must go before the serde::Serialize attribute
#[derive(TypedBuilder, Serialize)]
#[builder(
    doc,
    field_defaults(default, setter(strip_option, into)),
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
        doc = "A unique identifier for the item. It allows Alfred to learn about the item for subsequent sorting and ordering of the user's actioned results."
    ))]
    id: Option<String>,
    #[builder(setter(
        !strip_option,
        doc = "The title displayed in the result row. There are no options for this element and it is essential that this element is populated."
    ))]
    title: String,
    #[builder(setter(doc = "The subtitle displayed in the result row."))]
    subtitle: Option<String>,
    #[builder(
        setter(
            doc = "The argument which is passed through the workflow to the connected output action."
        ),
        via_mutators
    )]
    arg: Option<Vec<String>>,
    #[builder(setter(doc = "The icon displayed in the result row."))]
    icon: Option<Icon>,
    #[builder(
        default = Some(true),
        setter(
            doc = "If the `item` is valid or not. If an `item` is valid then Alfred will action it when the user presses return. If the `item` is not valid, Alfred will do nothing. This allows you to intelligently prevent Alfred from actioning a result based on the current `{query}` passed into your script."
        )
    )]
    valid: Option<bool>,
    #[builder(setter(
        doc = "The `match` field enables you to define what Alfred matches against when the workflow is set to \"Alfred Filters Results\". If `match` is present, it fully replaces matching on the title property."
    ))]
    r#match: Option<String>,
    #[builder(setter(
        doc = "An optional but recommended string you can provide to populate into Alfred's search field if the user auto-complete's the selected result (⇥ by default)."
    ))]
    autocomplete: Option<String>,
    #[builder(
        default = Some(Type::Default),
        setter(doc = "By specifying `\"type\": \"file\"`, Alfred treats your result as a file on your system. This allows the user to perform actions on the file like they can with Alfred's standard file filters.")
    )]
    r#type: Option<Type>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon() {
        let items = vec![
            Icon::builder().path("path/to/icon.png").build(),
            Icon::builder()
                .r#type(IconType::FileIcon)
                .path("~/Desktop")
                .build(),
            Icon::builder()
                .r#type(IconType::FileType)
                .path("com.apple.rtfd")
                .build(),
        ];
        insta::assert_json_snapshot!(items);
    }

    #[test]
    fn health_check() {
        let item = AlfredItem::builder()
            .id("42")
            .title("Hello, World!")
            .subtitle("This is a subtitle")
            .arg("arg1")
            .arg("arg2")
            .valid(false)
            .build();
        insta::assert_json_snapshot!(item);
    }

    #[test]
    fn num_of_arg() {
        let no_arg = AlfredItem::builder().title("Hello, World!").build();
        insta::assert_json_snapshot!(no_arg);
    }
}

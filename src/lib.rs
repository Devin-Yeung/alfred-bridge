#![doc(
    html_logo_url = "https://raw.githubusercontent.com/Devin-Yeung/alfred-bridge/master/docs/images/alfred-bridge-logo-square.png"
)]

use crate::variable::Variable;
use serde::Serialize;
use typed_builder::TypedBuilder;

pub mod item;
mod variable;

#[derive(Serialize, TypedBuilder)]
#[builder(
    doc,
    field_defaults(default),
    mutators(
        pub fn item(&mut self, item: item::AlfredItem) -> &mut Self {
            self.items.push(item);
            self
        }
    )
)]
pub struct AlfredOutput {
    #[builder(via_mutators)]
    items: Vec<item::AlfredItem>,
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<Variable>,
}

impl AlfredOutput {
    pub fn stdout(&self) -> Result<(), serde_json::Error> {
        println!("{}", serde_json::to_string(&self)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let output = AlfredOutput::builder()
            .item(
                item::AlfredItem::builder()
                    .title("Hello, World!")
                    .subtitle("This is a test subtitle.")
                    .arg("https://example.com")
                    .icon(
                        item::Icon::builder()
                            .r#type(item::IconType::FileIcon)
                            .path("icon.png")
                            .build(),
                    )
                    .build(),
            )
            .variables(
                Variable::builder()
                    .var("key", "value")
                    .var("key2", "value2")
                    .build(),
            )
            .build();

        output.stdout().unwrap(); // This will print the JSON to stdout
        insta::assert_json_snapshot!(output)
    }
}

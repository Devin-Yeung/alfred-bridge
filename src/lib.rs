#![doc(
    html_logo_url = "https://raw.githubusercontent.com/Devin-Yeung/alfred-bridge/master/docs/images/alfred-bridge-logo-square.png"
)]

use serde::Serialize;
use typed_builder::TypedBuilder;

mod item;
mod variable;

// re-export
pub use item::*;
pub use variable::*;

#[derive(Serialize, TypedBuilder)]
#[builder(
    doc,
    field_defaults(default, setter(strip_option)),
    mutators(
        pub fn item(&mut self, item: item::AlfredItem) -> &mut Self {
            self.items.push(item);
            self
        }
    )
)]
pub struct AlfredOutput {
    #[serde(rename = "skipknowledge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = Option::None)]
    skip_knowledge: Option<bool>,
    #[builder(via_mutators)]
    items: Vec<AlfredItem>,
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
    fn output_required_only() {
        let output = AlfredOutput::builder().build();
        insta::assert_json_snapshot!(output)
    }

    #[test]
    fn test_output() {
        let output = AlfredOutput::builder()
            .skip_knowledge(true)
            .item(
                AlfredItem::builder()
                    .title("Hello, World!")
                    .subtitle("This is a test subtitle.")
                    .arg("https://example.com")
                    .icon(
                        Icon::builder()
                            .r#type(IconType::FileIcon)
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

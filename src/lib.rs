use serde::Serialize;
use typed_builder::TypedBuilder;

pub mod item;

#[derive(Serialize, TypedBuilder)]
#[builder(
    doc,
    field_defaults(default),
    mutators(
        fn item(&mut self, item: item::AlfredItem) -> &mut Self {
            self.items.push(item);
            self
        }
    )
)]
pub struct AlfredOutput {
    #[builder(via_mutators)]
    items: Vec<item::AlfredItem>,
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
            .build();

        output.stdout().unwrap(); // This will print the JSON to stdout
        insta::assert_json_snapshot!(output)
    }
}

use serde::Serialize;
use typed_builder::TypedBuilder;

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

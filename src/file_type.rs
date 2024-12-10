use serde::Serialize;

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

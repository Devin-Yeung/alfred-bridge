#[cfg(test)]
use indexmap::IndexMap as Map;
use serde::Serialize;
#[cfg(not(test))]
use std::collections::HashMap as Map;
use typed_builder::TypedBuilder;

#[repr(transparent)]
#[derive(TypedBuilder)]
#[builder(
    doc,
    field_defaults(default, setter(into)),
    mutators(
        pub fn var(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
            self.variables.insert(key.into(), value.into());
            self
        }
    )
)]

pub struct Variable {
    #[builder(via_mutators)]
    variables: Map<String, String>,
}

impl Serialize for Variable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.variables.serialize(serializer)
    }
}

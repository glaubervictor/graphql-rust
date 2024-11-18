use async_graphql::{Error, ErrorExtensions, Value, Name};
use validator::ValidationErrors;
use indexmap::IndexMap;

pub trait IntoGraphQLError {
    fn into_graphql_error(self) -> Error;
}

impl IntoGraphQLError for ValidationErrors {
    fn into_graphql_error(self) -> Error {
        let mut extensions = IndexMap::new();

        for (field, errors) in self.field_errors() {
            let messages: Vec<String> = errors
                .iter()
                .map(|e| e.message.clone().unwrap_or_default().to_string())
                .collect();
            extensions.insert(Name::new(field), Value::from(messages));
        }

        let mut err = Error::new("Erro de validação");

        err = err.extend_with(|_, extensions_mut| {
            extensions_mut.set("validationErrors".to_string(), Value::Object(extensions));
        });

        err
    }
}

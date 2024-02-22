use crate::error::Result;
use serde::Deserialize;
use validator::{Validate, ValidationErrors, ValidationErrorsKind};

#[derive(Deserialize, Debug, Clone, Validate, Default)]
pub struct StorePageRequest {
    #[validate(length(min = 1, message = "The name is a required field."))]
    pub name: String,

    #[validate(length(min = 1, message = "The identifier is a required field."))]
    pub identifier: String,

    pub component_content: Vec<ComponentContent>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ComponentContent {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub content: String,
}

impl StorePageRequest {
    pub fn validate_errors(&self) -> Result<ValidationErrors> {
        let validation_error_list = match self.validate() {
            Ok(_) => ValidationErrors::new(),
            Err(errors) => errors,
        };

        for (_field_name, error) in validation_error_list.errors() {
            match &error {
                ValidationErrorsKind::Field(field_errors) => {
                    for _field_error in field_errors {
                        // IDea here is to add it into some kind of Error Response
                        // so we can return JSON struct with status code

                        // let message = match &field_error.message {
                        //     Some(message) => message,
                        //     None => continue,
                        // };
                    }
                }
                ValidationErrorsKind::Struct(_) => continue,
                ValidationErrorsKind::List(_) => continue,
            }
        }

        Ok(validation_error_list)
    }
}

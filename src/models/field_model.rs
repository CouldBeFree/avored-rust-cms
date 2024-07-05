use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};

use super::Pagination;

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct FieldModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub field_type: String,
    pub field_data: Option<Vec<FieldDataModel>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct FieldDataModel {
    pub label: String,
    pub value: String
}

impl TryFrom<Object> for FieldModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<FieldModel> {
        let id = match val.get("id") {
            Some(val) => {
                
                match val.clone() {
                    Value::Thing(v) => {
                        let id = v.id;
                        id.to_string()
                    }
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };
        let name = match val.get("name") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };

        let identifier = match val.get("identifier") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };

        let field_type = match val.get("field_type") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };


        let created_at = match val.get("created_at") {
            Some(val) => {
                
                match val.clone() {
                    Value::Datetime(v) => v,
                    _ => Datetime::default(),
                }
            }
            None => Datetime::default(),
        };
        let updated_at = match val.get("updated_at") {
            Some(val) => {
                
                match val.clone() {
                    Value::Datetime(v) => v,
                    _ => Datetime::default(),
                }
            }
            None => Datetime::default(),
        };

        let created_by = match val.get("created_by") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };

        let updated_by = match val.get("updated_by") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };

        let field_data = match val.get("field_data") {
            Some(val) => {
                
                match val.clone() {
                    Value::Array(v) => {
                        let mut arr = Vec::new();

                        for array in v.into_iter() {
                            let object = match array.clone() {
                                Value::Object(v) => v,
                                _ => surrealdb::sql::Object::default(),
                            };

                            let field_data_model: FieldDataModel = object.try_into()?;

                            arr.push(field_data_model)
                        }
                        arr
                    }
                    _ => Vec::new(),
                }
            }
            None => Vec::new(),
        };

        // let field_data = None;

        Ok(FieldModel {
            id,
            name,
            identifier,
            field_type,
            field_data: Some(field_data),
            created_at,
            updated_at,
            created_by,
            updated_by,
        })
    }
}


impl TryFrom<Object> for FieldDataModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<FieldDataModel> {
        let label = match val.get("label") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };
        let value = match val.get("value") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };


        Ok(FieldDataModel {
            label,
            value
        })
    }
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableFieldModel {
    pub name: String,
    pub identifier: String,
    pub field_type: String,
    pub field_data: Option<Vec<CreatableFieldDataModel>>,
    pub logged_in_username: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CreatableFieldDataModel {
    pub label: String,
    pub value: String,
}
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableFieldModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub field_type: String,
    pub field_data: Option<Vec<UpdatableFieldDataModel>>,
    pub logged_in_username: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct FieldPagination {
    pub data: Vec<FieldModel>,
    pub pagination: Pagination,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableFieldDataModel {
    pub label: String,
    pub value: String,
}
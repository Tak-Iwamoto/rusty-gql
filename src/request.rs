use serde::{Deserialize, Serialize};

use crate::container::ContextData;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(default)]
    pub query_doc: String,
    #[serde(default)]
    pub operation_name: Option<String>,
    #[serde(skip)]
    pub context_value: Option<ContextData>,
}

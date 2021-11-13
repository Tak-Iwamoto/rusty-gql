use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(default)]
    pub query_doc: String,
    #[serde(default)]
    pub operation_name: Option<String>,
}

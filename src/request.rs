use serde::{Deserialize, Serialize};

use crate::variables::Variables;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub operation_name: Option<String>,

    #[serde(default)]
    pub variables: Variables,
}

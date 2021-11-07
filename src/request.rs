use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(default)]
    query_doc: String,
    #[serde(default)]
    operation_name: Option<String>,
    #[serde(skip)]
    context_value: Option<String>,
}

pub struct RequestContextValue {

}

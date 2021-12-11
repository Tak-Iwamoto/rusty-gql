use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::GqlValue;

#[derive(Deserialize, Serialize, Default)]
pub struct Variables(BTreeMap<String, GqlValue>);

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::GqlValue;

#[derive(Serialize, Clone, Default, Debug)]
pub struct Variables(pub BTreeMap<String, GqlValue>);

impl<'de> Deserialize<'de> for Variables {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(
            <Option<BTreeMap<String, GqlValue>>>::deserialize(deserializer)?.unwrap_or_default(),
        ))
    }
}

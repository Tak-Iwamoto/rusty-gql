mod field_file;
mod operation_mod_file;

use futures_util::future::try_join_all;
use rusty_gql::{self, GqlField, OperationType};
use std::{collections::BTreeMap, io::Error};

use self::{field_file::FieldFile, operation_mod_file::OperationGqlModFile};

use super::build_file;

pub async fn create_operation_files(
    operations: &BTreeMap<String, GqlField>,
    operation_type: OperationType,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let base_path = operation_type.to_string().to_lowercase();

    for (_, field) in operations.iter() {
        let task = build_file(FieldFile {
            def: field,
            base_path: base_path.to_string(),
        });
        futures.push(task);
    }

    build_file(OperationGqlModFile {
        operation_type,
        operations,
    })
    .await?;

    try_join_all(futures).await
}

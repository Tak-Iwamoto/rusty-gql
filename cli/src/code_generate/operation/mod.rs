mod field_file;
mod operation_mod_file;

use futures_util::future::try_join_all;
use rusty_gql::{self, GqlField, OperationType};
use std::{collections::BTreeMap, io::Error};

use self::{field_file::FieldFile, operation_mod_file::OperationModFile};

use super::{file_path_str, create_file};

pub async fn create_operation_files(
    operations: &BTreeMap<String, GqlField>,
    operation_type: OperationType,
    base_path: &str,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    for (_, field) in operations.iter() {
        let task = create_file(FieldFile {
            def: field,
            path: file_path_str(vec![
                base_path,
                &operation_type.to_string().to_lowercase(),
                &field.name,
            ]),
        });
        futures.push(task);
    }

    create_file(OperationModFile {
        operation_type: operation_type.clone(),
        operations,
        path: file_path_str(vec![
            base_path,
            &operation_type.to_string().to_lowercase(),
            "mod",
        ]),
    })
    .await?;

    try_join_all(futures).await
}

mod field_file;
mod operation_mod_file;

use futures_util::future::try_join_all;
use heck::ToSnakeCase;
use rusty_gql::{self, GqlField, OperationType};
use std::{collections::BTreeMap, io::Error};

use self::{field_file::FieldFile, operation_mod_file::OperationModFile};

use super::{create_file, file_path_str};

pub async fn create_operation_files(
    operations: &BTreeMap<String, GqlField>,
    operation_type: OperationType,
    base_path: &str,
    interface_names: &Vec<String>,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();

    for (_, field) in operations.iter() {
        let filename = file_path_str(vec![
            base_path,
            &operation_type.to_string().to_lowercase(),
            &field.name.to_snake_case(),
        ]);
        let task = create_file(FieldFile {
            def: field,
            path: filename,
            interface_names: &interface_names,
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
        interface_names: &interface_names,
    })
    .await?;

    try_join_all(futures).await
}

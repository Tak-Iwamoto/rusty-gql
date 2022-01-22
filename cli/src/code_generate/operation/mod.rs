mod field_file;
mod operation_mod_file;

use futures_util::future::try_join_all;
use heck::ToSnakeCase;
use rusty_gql::{self, GqlField, OperationType};
use std::{collections::HashMap, io::Error};

use self::{field_file::FieldFile, operation_mod_file::OperationModFile};

use super::{create_file, path_str};

pub async fn create_operation_files(
    operations: &HashMap<String, GqlField>,
    operation_type: OperationType,
    base_path: &str,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();

    for (_, field) in operations.iter() {
        let filename = field.name.to_snake_case();
        let path = path_str(
            vec![
                base_path,
                &operation_type.to_string().to_lowercase(),
                &filename,
            ],
            true,
        );
        let task = create_file(FieldFile {
            file_name: filename,
            def: field,
            path,
        });
        futures.push(task);
    }

    create_file(OperationModFile {
        operation_type: operation_type.clone(),
        operations,
        path: path_str(
            vec![base_path, &operation_type.to_string().to_lowercase(), "mod"],
            true,
        ),
    })
    .await?;

    try_join_all(futures).await
}

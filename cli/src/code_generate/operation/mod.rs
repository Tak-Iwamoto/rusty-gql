mod field_file;
mod operation_mod_file;

use futures_util::future::try_join_all;
use rusty_gql::{self, GqlField, OperationType};
use std::{collections::BTreeMap, io::Error};

use self::{field_file::FieldFile, operation_mod_file::OperationGqlModFile};

use super::create_file;

pub async fn create_operation_files(
    operations: &BTreeMap<String, GqlField>,
    operation_type: OperationType,
    base_path: &String,
) -> Result<Vec<()>, Error> {
    let field_base_path = format!(
        "{}/{}",
        base_path,
        operation_type.clone().to_string().to_lowercase()
    );
    let mut futures = Vec::new();
    for (_, field) in operations.iter() {
        let task = create_file(FieldFile {
            def: field,
            base_path: field_base_path.to_string(),
        });
        futures.push(task);
    }

    create_file(OperationGqlModFile {
        operation_type,
        operations,
        base_path: base_path.to_string(),
    })
    .await?;

    try_join_all(futures).await
}

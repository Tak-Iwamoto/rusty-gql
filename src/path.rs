#[derive(Clone, Debug)]
pub struct GraphQLPath {
    pub prev: Option<Box<GraphQLPath>>,
    pub key: String,
    pub parent_name: String,
}

impl Default for GraphQLPath {
    fn default() -> Self {
        Self {
            prev: None,
            key: String::from(""),
            parent_name: String::from("Query"),
        }
    }
}

impl GraphQLPath {
    pub fn prev(mut self, prev: Option<GraphQLPath>) -> Self {
        self.prev = prev.map_or(None, |p| Some(Box::new(p)));
        self
    }

    pub fn key(mut self, key: String) -> Self {
        self.key = key;
        self
    }

    pub fn parent_name(mut self, parent_name: String) -> Self {
        self.parent_name = parent_name;
        self
    }

    // pub fn add_path<'a>(
    //     &self,
    //     prev_path: Option<GraphQLPath>,
    //     schema: &'a Schema<'a>,
    //     operation: &'a Operation<'a>,
    // ) -> GraphQLPath {
    //     match prev_path {
    //         Some(prev) => {
    //             GraphQLPath {
    //                 prev: Some(Box::new(prev)),
    //                 key: self.key,
    //                 parent_name:
    //             }
    //         },
    //         None => todo!(),
    //     }
    // }

    // fn get_parent_name<'a>(
    //     &self,
    //     prev_path: Option<GraphQLPath>,
    //     schema: &'a Schema<'a>,
    //     operation: &'a Operation<'a>,
    // ) -> &str {
    //     let root_fieldname = operation.definition.root_field.name;

    //     match root_fieldname {
    //         "query" => return "Query",
    //         "mutation" => return "Mutation",
    //         "subscription" => return "Subscription",
    //         _ => {
    //             if schema.queries.contains_key(root_fieldname) {
    //                 return "Query";
    //             } else if schema.mutations.contains_key(root_fieldname) {
    //                 return "Mutation";
    //             } else if schema.subscriptions.contains_key(root_fieldname) {
    //                 return "Subscription";
    //             } else {
    //                 match prev_path {
    //                     Some(prev) => {
    //                         return prev.key.as_str();
    //                     }
    //                     None => {
    //                         unreachable!()
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}

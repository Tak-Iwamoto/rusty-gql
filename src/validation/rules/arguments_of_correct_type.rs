// use graphql_parser::schema::Value;

// use crate::validation::visitor::{ValidationContext, Visitor};

// pub struct ArgumentsOfCorrectType<'a> {
//     pub current_args: Option<Vec<(String, Value<'a, String>)>>,
// }

// impl<'a> Visitor<'a> for ArgumentsOfCorrectType<'a> {
//     fn enter_directive(
//         &mut self,
//         _ctx: &mut ValidationContext,
//         directive: &'a graphql_parser::schema::Directive<'a, String>,
//     ) {
//         self.current_args = Some(directive.arguments.clone());
//     }

//     fn exit_directive(
//         &mut self,
//         _ctx: &mut ValidationContext,
//         _directive: &'a graphql_parser::schema::Directive<'a, String>,
//     ) {
//         self.current_args = None;
//     }

//     fn enter_field(
//         &mut self,
//         _ctx: &mut ValidationContext,
//         field: &'a graphql_parser::query::Field<'a, String>,
//     ) {
//         self.current_args = Some(field.arguments.clone());
//     }

//     fn exit_field(
//         &mut self,
//         _ctx: &mut ValidationContext,
//         _field: &'a graphql_parser::query::Field<'a, String>,
//     ) {
//         self.current_args = None;
//     }

//     fn enter_argument(
//         &mut self,
//         _ctx: &mut ValidationContext,
//         arg: &'a (String, Value<'a, String>),
//     ) {
//         let (arg_name, arg_value) = &arg;

//         if let Some(target_arg) = self
//             .current_args
//             .and_then(|args| args.iter().find(|a| a.0 == arg_name.clone()))
//         {}
//     }
// }

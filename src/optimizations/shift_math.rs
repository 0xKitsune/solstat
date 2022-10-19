// // use std::collections::HashMap;
// // use std::vec;

// // use crate::parser::solga_helpers::{
// //     extract_expressions_from_statement, parse_file_for_source_unit,
// // };

// use solang_parser::pt::{ContractPart, Expression, Loc, Type};
// use solang_parser::{self, pt::SourceUnit, pt::SourceUnitPart};

// use crate::ast::parse::extract_targets_from_node;

// ///Identifiy opportunities to pack structs to save gas
// pub fn analyze_for_mul_2_optimization(source_unit: SourceUnit) -> HashMap<Loc, Loc> {
//     let mut optimization_locations: HashMap<Loc, Loc> = HashMap::new();

//     //for each source unit part
//     for source_unit_part in source_unit.0 {
//         //check the file for contract definitions
//         if let SourceUnitPart::ContractDefinition(contract_definition) = source_unit_part {
//             //check each contract definition part
//             for part in contract_definition.parts {
//                 //if the contract part is a function definition
//                 if let ContractPart::FunctionDefinition(function_definition) = part {
//                     //if there is function body
//                     if function_definition.body.is_some() {
//                         //get the function body statement type
//                         let function_body_statement = function_definition.body.unwrap();

//                         //extract the expressions from the function body statement
//                         let expressions = extract_targets_from_node(function_body_statement);

//                         //for each extracted expression, look for instances of expr*2 or 2*expr
//                         for expression in expressions {
//                             if let Expression::Multiply(loc, box_expression, box_expression_1) =
//                                 expression
//                             {
//                                 //create a boolean to determine if mul_2 is present
//                                 let mut mul_2: bool = false;

//                                 //if the first expression is a number literal with a value of "2"
//                                 if let Expression::NumberLiteral(_, val_string, _) = *box_expression
//                                 {
//                                     if val_string == "2".to_string() {
//                                         //set mul_2 to true
//                                         mul_2 = true;
//                                     }
//                                 }

//                                 //else if  second expression is a number literal with a value of "2"
//                                 if let Expression::NumberLiteral(_, val_string, _) =
//                                     *box_expression_1
//                                 {
//                                     if val_string == "2".to_string() {
//                                         //set mul_2 to true
//                                         mul_2 = true;
//                                     }
//                                 }

//                                 //if mul_2 is true, push the location of the optimization match
//                                 if mul_2 {
//                                     optimization_locations.insert(loc, loc);
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     optimization_locations
// }

// #[test]
// fn test_analyze_for_mul_2_optimization() {
//     let file_contents = r#"

//     contract Contract0 {

//         function mul2(uint256 a, uint256 b) public pure {
//             uint256 a = 10 * 2;

//             uint256 b = 2 * a;
//             uint256 c = a * b;

//             uint256 d = (a * b) * 2;
//         }
//     }
//     "#;

//     let source_unit = parse_file_for_source_unit(file_contents, 0);

//     let optimization_locations = analyze_for_mul_2_optimization(source_unit);

//     assert_eq!(optimization_locations.len(), 3)
// }

use std::collections::{HashMap, HashSet};

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};

pub fn memory_to_calldata_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let target_nodes =
        ast::extract_target_from_node(Target::FunctionDefinition, source_unit.into());

    //For each target node that was extracted, check for the optimization patterns
    for node in target_nodes {
        //Can unwrap because Target::FunctionDefinition will always be a contract part
        let contract_part = node.contract_part().unwrap();

        if let pt::ContractPart::FunctionDefinition(box_function_definition) = contract_part {
            let mut memory_args =
                get_function_definition_memory_args(box_function_definition.clone());

            if box_function_definition.body.is_some() {
                let assign_nodes = ast::extract_target_from_node(
                    Target::Assign,
                    box_function_definition.body.unwrap().into(),
                );

                for assign_node in assign_nodes {
                    //Can unwrap because Target::Assign will always be an expression
                    let expression = assign_node.expression().unwrap();

                    if let pt::Expression::Assign(_, box_expression, _) = expression {
                        //check if the left hand side is a variable
                        match *box_expression {
                            //if assignment is to variable
                            pt::Expression::Variable(identifier) => {
                                memory_args.remove(&identifier.name);
                            }

                            //if assignment is array subscript
                            pt::Expression::ArraySubscript(_, arr_subscript_box_expression, _) => {
                                if let pt::Expression::Variable(identifier) =
                                    *arr_subscript_box_expression
                                {
                                    //remove the variable name from the memory_args hashmap
                                    memory_args.remove(&identifier.name);
                                }
                            }

                            _ => {}
                        }
                    }
                }
            }

            //for each arg in memory args left, add it to the optimization locations
            for (_, loc) in memory_args {
                optimization_locations.insert(loc);
            }
        }
    }

    //Return the identified optimization locations
    optimization_locations
}

fn get_function_definition_memory_args(
    function_definition: Box<pt::FunctionDefinition>,
) -> HashMap<String, Loc> {
    let mut memory_args: HashMap<String, Loc> = HashMap::new();
    for option_param in function_definition.params {
        if option_param.1.is_some() {
            let param = option_param.1.unwrap();

            if param.storage.is_some() {
                let storage_location = param.storage.unwrap();

                if let pt::StorageLocation::Memory(loc) = storage_location {
                    if param.name.is_some() {
                        let name = param.name.unwrap();

                        memory_args.insert(name.name.clone(), loc);
                    }
                }
            }
        }
    }

    memory_args
}

#[test]
fn test_memory_to_calldata_optimization() {
    let file_contents = r#"
   
contract Contract1 {
    //loop with i++
    function memoryArray(uint256[] memory arr) public {
        uint256 j;
        for (uint256 i; i < arr.length; i++) {
            j = arr[i] + 10;
        }
    }

    //loop with i++
    function calldataArray(uint256[] calldata arr) public {
        uint256 j;
        for (uint256 i; i < arr.length; i++) {
            j = arr[i] + 10;
        }
    }

    //loop with i++
    function memoryArray2(uint256[] memory arr) public {
        uint256 j;
        for (uint256 i; i < arr.length; i++) {
            j = arr[i] + 10;
            arr[i] = j + 10;
        }
    }

    //loop with i++
    function memoryBytes(bytes memory byteArr) public {
        bytes j;
        for (uint256 i; i < arr.length; i++) {
            j = byteArr;
        }
    }

    //loop with i++
    function calldataBytes(bytes calldata byteArr) public {
        bytes j;
        for (uint256 i; i < arr.length; i++) {
            j = byteArr;
        }
    }


    //loop with i++
    function memoryBytes1(bytes memory byteArr) public {
        bytes j;
        for (uint256 i; i < arr.length; i++) {
            byteArr = j;
        }
    }

}
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = memory_to_calldata_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 2)
}

use std::collections::HashSet;

use solang_parser::pt::{Expression, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};

pub fn assign_update_array_optimization(source_unit: SourceUnit) -> HashSet<Loc> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let target_nodes = ast::extract_target_from_node(Target::Assign, source_unit.into());

    //For each target node that was extracted, check for the optimization patterns
    for node in target_nodes {
        //We can use unwrap because Target::Assign is an expression
        let expression = node.expression().unwrap();

        if let Expression::Assign(loc, box_expression, box_expression_1) = expression {
            if let Expression::ArraySubscript(
                _,
                array_subscrip_box_expression,
                option_array_subscrip_box_expression_1,
            ) = *box_expression
            {
                //get the variable name of the array
                if let Expression::Variable(identifier) = *array_subscrip_box_expression {
                    let array_identifier = identifier.name;

                    if option_array_subscrip_box_expression_1.is_some() {
                        let array_subscrip_box_expression_1 =
                            option_array_subscrip_box_expression_1.unwrap();

                        if let Expression::NumberLiteral(_, number, _) =
                            *array_subscrip_box_expression_1
                        {
                            let index_accessed = number;

                            match *box_expression_1 {
                                Expression::Add(_, _box_expression, _box_expression_1)
                                | Expression::Subtract(_, _box_expression, _box_expression_1)
                                | Expression::Divide(_, _box_expression, _box_expression_1)
                                | Expression::Multiply(_, _box_expression, _box_expression_1)
                                | Expression::Modulo(_, _box_expression, _box_expression_1)
                                | Expression::ShiftLeft(_, _box_expression, _box_expression_1)
                                | Expression::ShiftRight(_, _box_expression, _box_expression_1)
                                | Expression::BitwiseAnd(_, _box_expression, _box_expression_1)
                                | Expression::BitwiseOr(_, _box_expression, _box_expression_1)
                                | Expression::BitwiseXor(_, _box_expression, _box_expression_1) => {
                                    if let Expression::ArraySubscript(
                                        _,
                                        array_subscrip_box_expression,
                                        option_array_subscrip_box_expression_1,
                                    ) = *_box_expression
                                    {
                                        //get the variable name of the array
                                        if let Expression::Variable(identifier) =
                                            *array_subscrip_box_expression
                                        {
                                            let _array_identifier = identifier.name;
                                            if _array_identifier == array_identifier {
                                                if option_array_subscrip_box_expression_1.is_some()
                                                {
                                                    let array_subscrip_box_expression_1 =
                                                        option_array_subscrip_box_expression_1
                                                            .unwrap();

                                                    if let Expression::NumberLiteral(_, number, _) =
                                                        *array_subscrip_box_expression_1
                                                    {
                                                        let _index_accessed = number;

                                                        if _index_accessed == index_accessed {
                                                            optimization_locations.insert(loc);
                                                        }
                                                    }
                                                }
                                            }
                                        } else if let Expression::ArraySubscript(
                                            _,
                                            array_subscrip_box_expression,
                                            option_array_subscrip_box_expression_1,
                                        ) = *_box_expression_1
                                        {
                                            //get the variable name of the array
                                            if let Expression::Variable(identifier) =
                                                *array_subscrip_box_expression
                                            {
                                                let _array_identifier = identifier.name;
                                                if array_identifier == _array_identifier {
                                                    if option_array_subscrip_box_expression_1
                                                        .is_some()
                                                    {
                                                        let array_subscrip_box_expression_1 =
                                                            option_array_subscrip_box_expression_1
                                                                .unwrap();

                                                        if let Expression::NumberLiteral(
                                                            _,
                                                            number,
                                                            _,
                                                        ) = *array_subscrip_box_expression_1
                                                        {
                                                            let _index_accessed = number;

                                                            if _index_accessed == index_accessed {
                                                                optimization_locations.insert(loc);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    //Return the identified optimization locations
    optimization_locations
}

#[test]
fn test_assign_update_array_optimization() {
    let file_contents = r#"
    
    pragma solidity >= 0.8.0;
    contract Contract {


        uint256[] vals;
        ;

        constructor(){
            vals = new uint256[](100);
        }
        function update() public {
            vals[0] = vals[0]+1;
            vals[0]+=1;
        }
    }
 
    "#;
    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = assign_update_array_optimization(source_unit);
    assert_eq!(optimization_locations.len(), 1);
}

use std::{collections::HashSet, vec};

use solang_parser::pt;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Target {
    //Statement Targets
    Args,
    Return,
    Revert,
    RevertNamedArgs,
    Emit,
    Expression,
    VariableDefinition,
    Block,
    If,
    While,
    For,
    DoWhile,
    Try,

    //Expression Targets
    Add,
    And,
    ArrayLiteral,
    ArraySlice,
    ArraySubscript,
    Assign,
    AssignAdd,
    AssignAnd,
    AssignDivide,
    AssignModulo,
    AssignMultiply,
    AssignOr,
    AssignShiftLeft,
    AssignShiftRight,
    AssignSubtract,
    AssignXor,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    Complement,
    Delete,
    Divide,
    Equal,
    FunctionCall,
    FunctionCallBlock,
    Less,
    LessEqual,
    List,
    MemberAccess,
    Modulo,
    More,
    MoreEqual,
    Multiply,
    NamedFunctionCall,
    New,
    Not,
    NotEqual,
    Or,
    Parenthesis,
    PostDecrement,
    PostIncrement,
    PreIncrement,
    PreDecrement,
    ShiftLeft,
    ShiftRight,
    Subtract,
    Ternary,
    Type,
    Function,
    UnaryMinus,
    UnaryPlus,
    Unit,
    Power,
    BoolLiteral,
    NumberLiteral,
    RationalNumberLiteral,
    HexNumberLiteral,
    HexLiteral,
    StringLiteral,
    AddressLiteral,
    Variable,
    This,

    //Source Unit / Contract Part
    SourceUnit,
    ContractDefinition,
    EnumDefinition,
    EventDefinition,
    ErrorDefinition,
    FunctionDefinition,
    ImportDirective,
    PragmaDirective,
    StraySemicolon,
    StructDefinition,
    TypeDefinition,
    Using,

    //If there is no target that corresponds
    None,
}

pub fn new_targets(targets: Vec<Target>) -> HashSet<Target> {
    let mut target_set = HashSet::new();

    for target in targets {
        target_set.insert(target);
    }

    target_set
}

pub fn statement_as_target(statement: &pt::Statement) -> Target {
    match statement {
        pt::Statement::Args(_, _) => return Target::Args,
        pt::Statement::Return(_, _) => return Target::Return,
        pt::Statement::Revert(_, _, _) => return Target::Revert,
        pt::Statement::Emit(_, _) => return Target::Emit,
        pt::Statement::RevertNamedArgs(_, _, _) => return Target::RevertNamedArgs,
        pt::Statement::Expression(_, _) => return Target::Expression,
        pt::Statement::VariableDefinition(_, _, _) => return Target::VariableDefinition,
        pt::Statement::Block {
            loc: _,
            unchecked: _,
            statements: _,
        } => return Target::Block,
        pt::Statement::If(_, _, _, _) => return Target::If,
        pt::Statement::While(_, _, _) => return Target::While,
        pt::Statement::For(_, _, _, _, _) => return Target::For,
        pt::Statement::DoWhile(_, _, _) => return Target::DoWhile,
        pt::Statement::Try(_, _, _, _) => return Target::Try,
        _ => return Target::None,
    }
}

pub fn expression_as_target(expression: &pt::Expression) -> Target {
    match expression {
        pt::Expression::Add(_, _, _) => Target::Add,
        pt::Expression::And(_, _, _) => Target::And,
        pt::Expression::ArrayLiteral(_, _) => Target::ArrayLiteral,
        pt::Expression::ArraySlice(_, _, _, _) => Target::ArraySlice,
        pt::Expression::ArraySubscript(_, _, _) => Target::ArraySubscript,
        pt::Expression::Assign(_, _, _) => Target::Assign,
        pt::Expression::AssignAdd(_, _, _) => Target::AssignAdd,
        pt::Expression::AssignAnd(_, _, _) => Target::AssignAnd,
        pt::Expression::AssignDivide(_, _, _) => Target::AssignDivide,
        pt::Expression::AssignModulo(_, _, _) => Target::AssignModulo,
        pt::Expression::AssignMultiply(_, _, _) => Target::AssignMultiply,
        pt::Expression::AssignOr(_, _, _) => Target::AssignOr,
        pt::Expression::AssignShiftLeft(_, _, _) => Target::AssignShiftLeft,
        pt::Expression::AssignShiftRight(_, _, _) => Target::AssignShiftRight,
        pt::Expression::AssignSubtract(_, _, _) => Target::AssignSubtract,
        pt::Expression::AssignXor(_, _, _) => Target::AssignXor,
        pt::Expression::BitwiseAnd(_, _, _) => Target::BitwiseAnd,
        pt::Expression::BitwiseOr(_, _, _) => Target::BitwiseOr,
        pt::Expression::BitwiseXor(_, _, _) => Target::BitwiseXor,
        pt::Expression::Complement(_, _) => Target::Complement,
        pt::Expression::Delete(_, _) => Target::Delete,
        pt::Expression::Divide(_, _, _) => Target::Divide,
        pt::Expression::Equal(_, _, _) => Target::Equal,
        pt::Expression::FunctionCall(_, _, _) => Target::FunctionCall,
        pt::Expression::FunctionCallBlock(_, _, _) => Target::FunctionCallBlock,
        pt::Expression::Less(_, _, _) => Target::Less,
        pt::Expression::LessEqual(_, _, _) => Target::LessEqual,
        pt::Expression::List(_, _) => Target::List,
        pt::Expression::MemberAccess(_, _, _) => Target::MemberAccess,
        pt::Expression::Modulo(_, _, _) => Target::Modulo,
        pt::Expression::More(_, _, _) => Target::More,
        pt::Expression::MoreEqual(_, _, _) => Target::MoreEqual,
        pt::Expression::Multiply(_, _, _) => Target::Multiply,
        pt::Expression::NamedFunctionCall(_, _, _) => Target::NamedFunctionCall,
        pt::Expression::New(_, _) => Target::New,
        pt::Expression::Not(_, _) => Target::Not,
        pt::Expression::NotEqual(_, _, _) => Target::NotEqual,
        pt::Expression::Or(_, _, _) => Target::Or,
        pt::Expression::Parenthesis(_, _) => Target::Parenthesis,
        pt::Expression::PostDecrement(_, _) => Target::PostDecrement,
        pt::Expression::PostIncrement(_, _) => Target::PostIncrement,
        pt::Expression::ShiftLeft(_, _, _) => Target::ShiftLeft,
        pt::Expression::ShiftRight(_, _, _) => Target::ShiftRight,
        pt::Expression::Subtract(_, _, _) => Target::Subtract,
        pt::Expression::Ternary(_, _, _, _) => Target::Ternary,
        pt::Expression::Type(_, _) => Target::Type,
        pt::Expression::UnaryMinus(_, _) => Target::UnaryMinus,
        pt::Expression::UnaryPlus(_, _) => Target::UnaryPlus,
        pt::Expression::Unit(_, _, _) => Target::Unit,
        pt::Expression::PreIncrement(_, _) => Target::PreIncrement,
        pt::Expression::PreDecrement(_, _) => Target::PreDecrement,
        pt::Expression::Power(_, _, _) => Target::Power,
        pt::Expression::BoolLiteral(_, _) => Target::BoolLiteral,
        pt::Expression::NumberLiteral(_, _, _) => Target::NumberLiteral,
        pt::Expression::RationalNumberLiteral(_, _, _, _) => Target::RationalNumberLiteral,
        pt::Expression::HexNumberLiteral(_, _) => Target::HexNumberLiteral,
        pt::Expression::HexLiteral(_) => Target::HexLiteral,
        pt::Expression::StringLiteral(_) => Target::StringLiteral,
        pt::Expression::AddressLiteral(_, _) => Target::AddressLiteral,
        pt::Expression::Variable(_) => Target::Variable,
        pt::Expression::This(_) => Target::This,
    }
}

pub fn source_unit_part_as_target(source_unit_part: &pt::SourceUnitPart) -> Target {
    match source_unit_part {
        pt::SourceUnitPart::ContractDefinition(_) => Target::ContractDefinition,
        pt::SourceUnitPart::EnumDefinition(_) => Target::EnumDefinition,
        pt::SourceUnitPart::ErrorDefinition(_) => Target::ErrorDefinition,
        pt::SourceUnitPart::EventDefinition(_) => Target::EventDefinition,
        pt::SourceUnitPart::FunctionDefinition(_) => Target::FunctionDefinition,
        pt::SourceUnitPart::ImportDirective(_) => Target::ImportDirective,
        pt::SourceUnitPart::PragmaDirective(_, _, _) => Target::PragmaDirective,
        pt::SourceUnitPart::StraySemicolon(_) => Target::StraySemicolon,
        pt::SourceUnitPart::StructDefinition(_) => Target::StructDefinition,
        pt::SourceUnitPart::TypeDefinition(_) => Target::TypeDefinition,
        pt::SourceUnitPart::Using(_) => Target::Using,
        pt::SourceUnitPart::VariableDefinition(_) => Target::VariableDefinition,
    }
}
pub fn contract_part_as_target(contract_part: &pt::ContractPart) -> Target {
    match contract_part {
        pt::ContractPart::EnumDefinition(_) => Target::EnumDefinition,
        pt::ContractPart::ErrorDefinition(_) => Target::ErrorDefinition,
        pt::ContractPart::EventDefinition(_) => Target::EventDefinition,
        pt::ContractPart::FunctionDefinition(_) => Target::FunctionDefinition,
        pt::ContractPart::StraySemicolon(_) => Target::StraySemicolon,
        pt::ContractPart::StructDefinition(_) => Target::StructDefinition,
        pt::ContractPart::TypeDefinition(_) => Target::TypeDefinition,
        pt::ContractPart::Using(_) => Target::Using,
        pt::ContractPart::VariableDefinition(_) => Target::VariableDefinition,
    }
}

pub fn extract_target_from_node(target: Target, node: Node) -> Vec<Node> {
    let mut target_set = HashSet::new();
    target_set.insert(target);

    return walk_node_for_targets(&target_set, node);
}

pub fn extract_targets_from_node(targets: Vec<Target>, node: Node) -> Vec<Node> {
    let mut target_set = HashSet::new();

    for target in targets {
        target_set.insert(target);
    }

    return walk_node_for_targets(&target_set, node);
}

//Extract target ast node types from a parent node
pub fn walk_node_for_targets(targets: &HashSet<Target>, node: Node) -> Vec<Node> {
    let mut matches = vec![];

    if targets.contains(&node.as_target()) {
        matches.push(node.clone());
    }

    match node {
        Node::SourceUnit(source_unit) => {
            for source_unit_part in source_unit.0 {
                matches.append(&mut walk_node_for_targets(targets, source_unit_part.into()));
            }
        }
        Node::SourceUnitPart(source_unit_part) => match source_unit_part {
            pt::SourceUnitPart::ContractDefinition(box_contract_definition) => {
                //Walk the contract definition base for targets
                for base in box_contract_definition.base {
                    if base.args.is_some() {
                        let args = base.args.unwrap();

                        for arg in args {
                            matches.append(&mut walk_node_for_targets(targets, arg.into()));
                        }
                    }
                }

                //Walk the contract definition parts for targets
                for part in box_contract_definition.parts {
                    matches.append(&mut walk_node_for_targets(targets, part.into()));
                }
            }

            pt::SourceUnitPart::ErrorDefinition(box_error_definition) => {
                for error_parameter in box_error_definition.fields {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        error_parameter.ty.into(),
                    ));
                }
            }

            pt::SourceUnitPart::EventDefinition(box_event_definition) => {
                for event_parameter in box_event_definition.fields {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        event_parameter.ty.into(),
                    ));
                }
            }

            pt::SourceUnitPart::FunctionDefinition(box_function_definition) => {
                //Walk params for targets
                for (_, option_parameter) in box_function_definition.params {
                    if option_parameter.is_some() {
                        matches.append(&mut walk_node_for_targets(
                            targets,
                            option_parameter.unwrap().ty.into(),
                        ));
                    }
                }
                //Walk return params for targets
                for (_, option_parameter) in box_function_definition.returns {
                    if option_parameter.is_some() {
                        matches.append(&mut walk_node_for_targets(
                            targets,
                            option_parameter.unwrap().ty.into(),
                        ));
                    }
                }

                //Walk the function body for targets
                if box_function_definition.body.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        box_function_definition.body.unwrap().into(),
                    ));
                }
            }

            pt::SourceUnitPart::StructDefinition(box_struct_definition) => {
                for variable_declaration in box_struct_definition.fields {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        variable_declaration.ty.into(),
                    ));
                }
            }

            pt::SourceUnitPart::TypeDefinition(box_type_definition) => {
                matches.append(&mut walk_node_for_targets(
                    targets,
                    box_type_definition.ty.into(),
                ));
            }

            pt::SourceUnitPart::Using(box_using) => {
                if box_using.ty.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        box_using.ty.unwrap().into(),
                    ));
                }
            }
            pt::SourceUnitPart::VariableDefinition(box_variable_definition) => {
                matches.append(&mut walk_node_for_targets(
                    targets,
                    box_variable_definition.ty.into(),
                ));

                if box_variable_definition.initializer.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        box_variable_definition.initializer.unwrap().into(),
                    ));
                }
            }

            _ => {
                //Pragma Directive
                //Stray Semicolon
                //EnumDefinition
                //Import directive
            }
        },

        Node::ContractPart(contract_part) => match contract_part {
            pt::ContractPart::ErrorDefinition(box_error_definition) => {
                for error_parameter in box_error_definition.fields {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        error_parameter.ty.into(),
                    ));
                }
            }

            pt::ContractPart::EventDefinition(box_event_definition) => {
                for event_parameter in box_event_definition.fields {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        event_parameter.ty.into(),
                    ));
                }
            }

            pt::ContractPart::FunctionDefinition(box_function_definition) => {
                //Walk params for targets
                for (_, option_parameter) in box_function_definition.params {
                    if option_parameter.is_some() {
                        matches.append(&mut walk_node_for_targets(
                            targets,
                            option_parameter.unwrap().ty.into(),
                        ));
                    }
                }
                //Walk return params for targets
                for (_, option_parameter) in box_function_definition.returns {
                    if option_parameter.is_some() {
                        matches.append(&mut walk_node_for_targets(
                            targets,
                            option_parameter.unwrap().ty.into(),
                        ));
                    }
                }

                //Walk the function body for targets
                if box_function_definition.body.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        box_function_definition.body.unwrap().into(),
                    ));
                }
            }

            pt::ContractPart::StructDefinition(box_struct_definition) => {
                for variable_declaration in box_struct_definition.fields {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        variable_declaration.ty.into(),
                    ));
                }
            }

            pt::ContractPart::TypeDefinition(box_type_definition) => {
                matches.append(&mut walk_node_for_targets(
                    targets,
                    box_type_definition.ty.into(),
                ));
            }

            pt::ContractPart::Using(box_using) => {
                if box_using.ty.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        box_using.ty.unwrap().into(),
                    ));
                }
            }
            pt::ContractPart::VariableDefinition(box_variable_definition) => {
                matches.append(&mut walk_node_for_targets(
                    targets,
                    box_variable_definition.ty.into(),
                ));

                if box_variable_definition.initializer.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        box_variable_definition.initializer.unwrap().into(),
                    ));
                }
            }

            _ => {
                //Stray Semicolon
                //EnumDefinition
            }
        },

        Node::Statement(statement) => match statement {
            pt::Statement::Args(_, named_arguments) => {
                for argument in named_arguments {
                    matches.append(&mut walk_node_for_targets(targets, argument.expr.into()));
                }
            }

            pt::Statement::Return(_, option_expression) => {
                if option_expression.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        option_expression.unwrap().into(),
                    ));
                }
            }

            pt::Statement::Revert(_, _, vec_expression) => {
                for expression in vec_expression {
                    matches.append(&mut walk_node_for_targets(targets, expression.into()));
                }
            }

            pt::Statement::Emit(_, expression) => {
                matches.append(&mut walk_node_for_targets(targets, expression.into()));
            }

            pt::Statement::RevertNamedArgs(_, _, vec_named_arguments) => {
                for named_argument in vec_named_arguments {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        named_argument.expr.into(),
                    ));
                }
            }

            pt::Statement::Expression(_, expression) => {
                matches.append(&mut walk_node_for_targets(targets, expression.into()));
            }

            pt::Statement::VariableDefinition(_, variable_declaration, option_expression) => {
                matches.append(&mut walk_node_for_targets(
                    targets,
                    variable_declaration.ty.into(),
                ));

                if option_expression.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        option_expression.unwrap().into(),
                    ));
                }
            }

            pt::Statement::Block {
                loc: _,
                unchecked: _,
                statements,
            } => {
                for statement in statements {
                    matches.append(&mut walk_node_for_targets(targets, statement.into()));
                }
            }

            pt::Statement::If(_, expression, box_statement, option_box_statement) => {
                matches.append(&mut walk_node_for_targets(targets, expression.into()));

                matches.append(&mut walk_node_for_targets(targets, box_statement.into()));

                if option_box_statement.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        option_box_statement.unwrap().into(),
                    ));
                }
            }

            pt::Statement::While(_, expression, box_statement) => {
                matches.append(&mut walk_node_for_targets(targets, expression.into()));

                matches.append(&mut walk_node_for_targets(targets, box_statement.into()));
            }

            pt::Statement::For(
                _,
                option_box_statement,
                option_box_expression,
                option_box_statement_1,
                option_box_statement_2,
            ) => {
                if option_box_statement.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        option_box_statement.unwrap().into(),
                    ));
                }

                if option_box_expression.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        option_box_expression.unwrap().into(),
                    ));
                }

                if option_box_statement_1.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        option_box_statement_1.unwrap().into(),
                    ));
                }
                if option_box_statement_2.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        option_box_statement_2.unwrap().into(),
                    ));
                }
            }

            pt::Statement::DoWhile(_, box_statement, expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_statement.into()));

                matches.append(&mut walk_node_for_targets(targets, expression.into()));
            }

            pt::Statement::Try(_, expression, option_paramlist_box_statement, _) => {
                matches.append(&mut walk_node_for_targets(targets, expression.into()));

                if option_paramlist_box_statement.is_some() {
                    let (paramlist, box_statement) = option_paramlist_box_statement.unwrap();

                    for (_, option_param) in paramlist {
                        if option_param.is_some() {
                            matches.append(&mut walk_node_for_targets(
                                targets,
                                option_param.unwrap().ty.into(),
                            ));
                        }
                    }

                    matches.append(&mut walk_node_for_targets(targets, box_statement.into()));
                }
            }

            _ => {
                //Assembly block
                //Continue
                //Break
            }
        },

        Node::Expression(expression) => match expression {
            pt::Expression::Add(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::And(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::ArrayLiteral(_, vec_expression) => {
                for expression in vec_expression {
                    matches.append(&mut walk_node_for_targets(targets, expression.into()));
                }
            }

            pt::Expression::ArraySlice(
                _,
                box_expression,
                option_box_expression,
                option_box_expression_1,
            ) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                if option_box_expression.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        option_box_expression.unwrap().into(),
                    ));
                }

                if option_box_expression_1.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        option_box_expression_1.unwrap().into(),
                    ));
                }
            }
            pt::Expression::ArraySubscript(_, box_expression, option_box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                if option_box_expression.is_some() {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        option_box_expression.unwrap().into(),
                    ));
                }
            }

            pt::Expression::Assign(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::AssignAdd(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }
            pt::Expression::AssignAnd(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::AssignDivide(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::AssignModulo(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::AssignMultiply(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::AssignOr(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }
            pt::Expression::AssignShiftLeft(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::AssignShiftRight(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::AssignSubtract(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }
            pt::Expression::AssignXor(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::BitwiseAnd(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::BitwiseOr(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::BitwiseXor(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::Complement(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }

            pt::Expression::Delete(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }
            pt::Expression::Divide(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::Equal(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::FunctionCall(_, box_expression, vec_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                for expression in vec_expression {
                    matches.append(&mut walk_node_for_targets(targets, expression.into()));
                }
            }

            pt::Expression::FunctionCallBlock(_, box_expression, box_statement) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                matches.append(&mut walk_node_for_targets(targets, box_statement.into()));
            }

            pt::Expression::Less(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::LessEqual(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::List(_, parameter_list) => {
                for (_, option_parameter) in parameter_list {
                    if option_parameter.is_some() {
                        let parameter = option_parameter.unwrap();
                        matches.append(&mut walk_node_for_targets(targets, parameter.ty.into()));
                    }
                }
            }

            pt::Expression::MemberAccess(_, box_expression, _) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }

            pt::Expression::Modulo(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::More(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }
            pt::Expression::MoreEqual(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::Multiply(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::NamedFunctionCall(_, box_expression, vec_named_argument) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                for named_argument in vec_named_argument {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        named_argument.expr.into(),
                    ));
                }
            }

            pt::Expression::New(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }
            pt::Expression::Not(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }
            pt::Expression::NotEqual(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::Or(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::Parenthesis(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }

            pt::Expression::PostDecrement(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }

            pt::Expression::PostIncrement(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }

            pt::Expression::ShiftLeft(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::ShiftRight(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }
            pt::Expression::Subtract(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::Ternary(_, box_expression, box_expression_1, box_expression_2) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));

                matches.append(&mut walk_node_for_targets(targets, box_expression_2.into()));
            }
            pt::Expression::Type(_, ty) => match ty {
                pt::Type::Mapping(_, box_expression, box_expression_1) => {
                    matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                    matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
                }

                pt::Type::Function {
                    params,
                    attributes,
                    returns,
                } => {
                    for param in params {
                        if param.1.is_some() {
                            matches.append(&mut walk_node_for_targets(
                                targets,
                                param.1.unwrap().ty.into(),
                            ));
                        }
                    }

                    for attribute in attributes {
                        match attribute {
                            pt::FunctionAttribute::BaseOrModifier(_, base) => {
                                if base.args.is_some() {
                                    for arg in base.args.unwrap() {
                                        matches.append(&mut walk_node_for_targets(
                                            targets,
                                            arg.into(),
                                        ));
                                    }
                                }
                            }

                            pt::FunctionAttribute::NameValue(_, _, expression) => {
                                matches
                                    .append(&mut walk_node_for_targets(targets, expression.into()));
                            }
                            _ => {}
                        }
                    }

                    if returns.is_some() {
                        let (parameter_list, function_attributes) = returns.unwrap();

                        for (_, option_parameter) in parameter_list {
                            if option_parameter.is_some() {
                                matches.append(&mut walk_node_for_targets(
                                    targets,
                                    option_parameter.unwrap().ty.into(),
                                ));
                            }
                        }

                        for attribute in function_attributes {
                            match attribute {
                                pt::FunctionAttribute::BaseOrModifier(_, base) => {
                                    if base.args.is_some() {
                                        for arg in base.args.unwrap() {
                                            matches.append(&mut walk_node_for_targets(
                                                targets,
                                                arg.into(),
                                            ));
                                        }
                                    }
                                }

                                pt::FunctionAttribute::NameValue(_, _, expression) => {
                                    matches.append(&mut walk_node_for_targets(
                                        targets,
                                        expression.into(),
                                    ));
                                }
                                _ => {}
                            }
                        }
                    }
                }

                _ => {}
            },

            pt::Expression::UnaryMinus(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }
            pt::Expression::UnaryPlus(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }

            pt::Expression::Unit(_, box_expression, _) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }
            _ => {
                //Address literal
                //Bool literal
                //Hex literal
                //Hex number literal
                //Number literal
                // Rational number literal
                //String literal
                //This
                //Variable
            }
        },
    }

    matches
}

impl Node {
    pub fn as_target(&self) -> Target {
        match self {
            Self::Expression(expression) => return expression_as_target(expression),
            Self::Statement(statement) => return statement_as_target(statement),
            Self::SourceUnit(_) => return Target::SourceUnit,
            Self::SourceUnitPart(source_unit_part) => {
                return source_unit_part_as_target(source_unit_part)
            }
            Self::ContractPart(contract_part) => return contract_part_as_target(contract_part),
        }
    }

    pub fn expression(self) -> Option<pt::Expression> {
        match self {
            Self::Expression(expression) => Some(expression),
            _ => None,
        }
    }

    pub fn statement(self) -> Option<pt::Statement> {
        match self {
            Self::Statement(statement) => Some(statement),
            _ => None,
        }
    }

    pub fn source_unit(self) -> Option<pt::SourceUnit> {
        match self {
            Self::SourceUnit(source_unit) => Some(source_unit),
            _ => None,
        }
    }

    pub fn source_unit_part(self) -> Option<pt::SourceUnitPart> {
        match self {
            Self::SourceUnitPart(source_unit_part) => Some(source_unit_part),
            _ => None,
        }
    }

    pub fn is_source_unit_part(&self) -> bool {
        if let Self::SourceUnitPart(_) = self {
            true
        } else {
            false
        }
    }

    pub fn contract_part(self) -> Option<pt::ContractPart> {
        match self {
            Self::ContractPart(contract_part) => Some(contract_part),
            _ => None,
        }
    }

    pub fn is_contract_part(&self) -> bool {
        if let Self::ContractPart(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Node {
    Statement(pt::Statement),
    Expression(pt::Expression),
    SourceUnit(pt::SourceUnit),
    SourceUnitPart(pt::SourceUnitPart),
    ContractPart(pt::ContractPart),
}

impl Into<Node> for pt::Statement {
    fn into(self) -> Node {
        Node::Statement(self)
    }
}

impl Into<Node> for Box<pt::Statement> {
    fn into(self) -> Node {
        Node::Statement(*self)
    }
}

impl Into<Node> for pt::Expression {
    fn into(self) -> Node {
        Node::Expression(self)
    }
}
impl Into<Node> for Box<pt::Expression> {
    fn into(self) -> Node {
        Node::Expression(*self)
    }
}

impl Into<Node> for pt::ContractPart {
    fn into(self) -> Node {
        Node::ContractPart(self)
    }
}

impl Into<Node> for pt::SourceUnitPart {
    fn into(self) -> Node {
        Node::SourceUnitPart(self)
    }
}

impl Into<Node> for pt::SourceUnit {
    fn into(self) -> Node {
        Node::SourceUnit(self)
    }
}

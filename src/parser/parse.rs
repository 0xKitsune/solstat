use std::{
    collections::{HashMap, HashSet},
    vec,
};

use solang_parser::pt::{self, Expression, NamedArgument, Statement};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Target {
    //Expression Targets

    //Statement Targets
}

impl Into<Target> for Node {
    fn into(self) -> Target {
        todo!()
    }
}

impl Into<Target> for pt::Statement {
    fn into(self) -> Target {
        todo!()
    }
}

impl Into<Target> for pt::Expression {
    fn into(self) -> Target {
        todo!()
    }
}

pub enum Node {
    Statement(pt::Statement),
    Expression(pt::Expression),
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
//Extract target ast node types from a parent node
pub fn extract_targets_from_node(targets: HashSet<Target>, node: Node) -> Vec<Node> {
    let mut matches = vec![];

    if targets.contains(&node.into()) {
        matches.push(node);
    }

    match node {
        Node::Statement(statement) => {
            match statement {
                pt::Statement::Args(_, named_arguments) => {
                    for argument in named_arguments {
                        matches.append(&mut extract_targets_from_node(
                            targets,
                            argument.expr.into(),
                        ));
                    }
                }

                pt::Statement::Return(_, option_expression) => {
                    if option_expression.is_some() {
                        matches.append(&mut extract_targets_from_node(
                            targets,
                            option_expression.unwrap().into(),
                        ));
                    }
                }

                pt::Statement::Revert(_, _, vec_expression) => {
                    for expression in vec_expression {
                        matches.append(&mut extract_targets_from_node(targets, expression.into()));
                    }
                }

                pt::Statement::Emit(_, expression) => {
                    matches.append(&mut extract_targets_from_node(targets, expression.into()));
                }

                pt::Statement::RevertNamedArgs(_, _, vec_named_arguments) => {
                    for named_argument in vec_named_arguments {
                        matches.append(&mut extract_targets_from_node(
                            targets,
                            named_argument.expr.into(),
                        ));
                    }
                }

                pt::Statement::Expression(_, expression) => {
                    matches.append(&mut extract_targets_from_node(targets, expression.into()));
                }

                pt::Statement::VariableDefinition(_, variable_declaration, option_expression) => {
                    matches.append(&mut extract_targets_from_node(
                        targets,
                        variable_declaration.ty.into(),
                    ));

                    if option_expression.is_some() {
                        matches.append(&mut extract_targets_from_node(
                            targets,
                            option_expression.unwrap().into(),
                        ));
                    }
                }

                pt::Statement::Block {
                    loc,
                    unchecked,
                    statements,
                } => {
                    for statement in statements {
                        matches.append(&mut extract_targets_from_node(targets, statement.into()));
                    }
                }

                pt::Statement::If(_, expression, box_statement, option_box_statement) => {
                    matches.append(&mut extract_targets_from_node(targets, expression.into()));

                    matches.append(&mut extract_targets_from_node(
                        targets,
                        box_statement.into(),
                    ));

                    if option_box_statement.is_some() {
                        matches.append(&mut extract_targets_from_node(
                            targets,
                            option_box_statement.unwrap().into(),
                        ));
                    }
                }

                pt::Statement::While(_, expression, box_statement) => {
                    matches.append(&mut extract_targets_from_node(targets, expression.into()));

                    matches.append(&mut extract_targets_from_node(
                        targets,
                        box_statement.into(),
                    ));
                }

                pt::Statement::For(
                    _,
                    option_box_statement,
                    option_box_expression,
                    option_box_statement_1,
                    option_box_statement_2,
                ) => {
                    if option_box_statement.is_some() {
                        matches.append(&mut extract_targets_from_node(
                            targets,
                            option_box_statement.unwrap().into(),
                        ));
                    }

                    if option_box_expression.is_some() {
                        matches.append(&mut extract_targets_from_node(
                            targets,
                            option_box_expression.unwrap().into(),
                        ));
                    }

                    if option_box_statement_1.is_some() {
                        matches.append(&mut extract_targets_from_node(
                            targets,
                            option_box_statement_1.unwrap().into(),
                        ));
                    }
                    if option_box_statement_2.is_some() {
                        matches.append(&mut extract_targets_from_node(
                            targets,
                            option_box_statement_2.unwrap().into(),
                        ));
                    }
                }

                pt::Statement::DoWhile(_, box_statement, expression) => {
                    matches.append(&mut extract_targets_from_node(
                        targets,
                        box_statement.into(),
                    ));

                    matches.append(&mut extract_targets_from_node(targets, expression.into()));
                }

                //------------------------------------
                pt::Statement::Try(_, expression, option_paramlist_box_statement, _) => {
                    matches.append(&mut extract_targets_from_node(targets, expression.into()));

                    if option_paramlist_box_statement.is_some() {
                        let (paramlist, box_statement) = option_paramlist_box_statement.unwrap();

                        for (_, option_param) in paramlist {
                            if option_param.is_some() {
                                matches.append(&mut extract_targets_from_node(
                                    targets,
                                    option_param.unwrap().ty.into(),
                                ));
                            }
                        }

                        matches.append(&mut extract_targets_from_node(
                            targets,
                            box_statement.into(),
                        ));
                    }
                }

                _ => {
                    //Assembly block
                    //Continue
                    //Break
                }
            }
        }
        Node::Expression(expression) => match expression {
            pt::Expression::Add(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));

                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::And(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));

                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::ArrayLiteral(_, vec_expression) => {
                for expression in vec_expression {
                    matches.append(&mut extract_targets_from_node(targets, expression.into()));
                }
            }

            pt::Expression::ArraySlice(
                _,
                box_expression,
                option_box_expression,
                option_box_expression_1,
            ) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));

                if option_box_expression.is_some() {
                    matches.append(&mut extract_targets_from_node(
                        targets,
                        option_box_expression.unwrap().into(),
                    ));
                }

                if option_box_expression_1.is_some() {
                    matches.append(&mut extract_targets_from_node(
                        targets,
                        option_box_expression_1.unwrap().into(),
                    ));
                }
            }
            pt::Expression::ArraySubscript(_, box_expression, option_box_expression) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));

                if option_box_expression.is_some() {
                    matches.append(&mut extract_targets_from_node(
                        targets,
                        option_box_expression.unwrap().into(),
                    ));
                }
            }

            pt::Expression::Assign(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::AssignAdd(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }
            pt::Expression::AssignAnd(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));

                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::AssignDivide(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::AssignModulo(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::AssignMultiply(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::AssignOr(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }
            pt::Expression::AssignShiftLeft(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::AssignShiftRight(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::AssignSubtract(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }
            pt::Expression::AssignXor(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::BitwiseAnd(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::BitwiseOr(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::BitwiseXor(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::Complement(_, box_expression) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
            }

            pt::Expression::Delete(_, box_expression) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
            }
            pt::Expression::Divide(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::Equal(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::FunctionCall(_, box_expression, vec_expression) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));

                for expression in vec_expression {
                    matches.append(&mut extract_targets_from_node(targets, expression.into()));
                }
            }

            pt::Expression::FunctionCallBlock(_, box_expression, box_statement) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));

                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_statement.into(),
                ));
            }

            pt::Expression::Less(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::LessEqual(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::List(_, parameter_list) => {
                for (_, option_parameter) in parameter_list {
                    if option_parameter.is_some() {
                        let parameter = option_parameter.unwrap();
                        matches
                            .append(&mut extract_targets_from_node(targets, parameter.ty.into()));
                    }
                }
            }

            pt::Expression::MemberAccess(_, box_expression, _) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
            }

            pt::Expression::Modulo(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::More(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }
            pt::Expression::MoreEqual(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::Multiply(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::NamedFunctionCall(_, box_expression, vec_named_argument) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));

                for named_argument in vec_named_argument {
                    matches.append(&mut extract_targets_from_node(
                        targets,
                        named_argument.expr.into(),
                    ));
                }
            }

            pt::Expression::New(_, box_expression) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
            }
            pt::Expression::Not(_, box_expression) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
            }
            pt::Expression::NotEqual(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::Or(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::Parenthesis(_, box_expression) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
            }

            pt::Expression::PostDecrement(_, box_expression) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
            }

            pt::Expression::PostIncrement(_, box_expression) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
            }

            pt::Expression::ShiftLeft(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::ShiftRight(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }
            pt::Expression::Subtract(_, box_expression, box_expression_1) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));
            }

            pt::Expression::Ternary(_, box_expression, box_expression_1, box_expression_2) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));

                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_1.into(),
                ));

                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression_2.into(),
                ));
            }
            pt::Expression::Type(_, ty) => match ty {
                pt::Type::Mapping(_, box_expression, box_expression_1) => {
                    matches.append(&mut extract_targets_from_node(
                        targets,
                        box_expression.into(),
                    ));

                    matches.append(&mut extract_targets_from_node(
                        targets,
                        box_expression_1.into(),
                    ));
                }

                pt::Type::Function {
                    params,
                    attributes,
                    returns,
                } => {
                    for param in params {
                        if param.1.is_some() {
                            matches.append(&mut extract_targets_from_node(
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
                                        matches.append(&mut extract_targets_from_node(
                                            targets,
                                            arg.into(),
                                        ));
                                    }
                                }
                            }

                            pt::FunctionAttribute::NameValue(_, _, expression) => {
                                matches.append(&mut extract_targets_from_node(
                                    targets,
                                    expression.into(),
                                ));
                            }
                            _ => {}
                        }
                    }

                    if returns.is_some() {
                        let (parameter_list, function_attributes) = returns.unwrap();

                        for (_, option_parameter) in parameter_list {
                            if option_parameter.is_some() {
                                matches.append(&mut extract_targets_from_node(
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
                                            matches.append(&mut extract_targets_from_node(
                                                targets,
                                                arg.into(),
                                            ));
                                        }
                                    }
                                }

                                pt::FunctionAttribute::NameValue(_, _, expression) => {
                                    matches.append(&mut extract_targets_from_node(
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
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
            }
            pt::Expression::UnaryPlus(_, box_expression) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
            }

            pt::Expression::Unit(_, box_expression, _) => {
                matches.append(&mut extract_targets_from_node(
                    targets,
                    box_expression.into(),
                ));
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

use std::{
    collections::{HashMap, HashSet},
    vec,
};

use solang_parser::pt::{self, Expression, Statement};

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
        Node::Expression(expression) => {}
    }

    matches
}

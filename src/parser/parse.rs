use std::{
    collections::{HashMap, HashSet},
    vec,
};

use solang_parser::pt;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum StatementTarget {}

impl Into<StatementTarget> for pt::Statement {
    fn into(self) -> StatementTarget {
        todo!()
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum ExpressionTarget {}

impl Into<ExpressionTarget> for pt::Statement {
    fn into(self) -> ExpressionTarget {
        todo!()
    }
}

pub fn extract_target_statements_from_statement(
    targets: HashSet<StatementTarget>,
    statement: pt::Statement,
) -> Vec<pt::Statement> {
    let mut matches = vec![];

    if targets.contains(&statement.clone().into()) {
        matches.push(statement);
    }

    match statement {
        pt::Statement::Block {
            loc: _,
            unchecked: _,
            statements,
        } => {
            for statement in statements {
                matches.append(&mut extract_target_statements_from_statement(
                    targets, statement,
                ));
            }
        }

        pt::Statement::If(_, _, box_statement, option_box_statement) => {
            matches.append(&mut extract_target_statements_from_option_box_statement(
                targets,
                Some(box_statement),
            ));

            matches.append(&mut extract_target_statements_from_option_box_statement(
                targets,
                option_box_statement,
            ));
        }

        pt::Statement::While(_, _, box_statement) => {
            matches.append(&mut extract_target_statements_from_option_box_statement(
                targets,
                Some(box_statement),
            ));
        }

        pt::Statement::For(
            _,
            option_box_statement,
            _,
            option_box_statement_1,
            option_box_statement_2,
        ) => {
            matches.append(&mut extract_target_statements_from_option_box_statement(
                targets,
                option_box_statement,
            ));

            matches.append(&mut extract_target_statements_from_option_box_statement(
                targets,
                option_box_statement_1,
            ));

            matches.append(&mut extract_target_statements_from_option_box_statement(
                targets,
                option_box_statement_2,
            ));
        }
        pt::Statement::DoWhile(_, box_statement, expression) => {
            matches.append(&mut extract_target_statements_from_option_box_statement(
                targets,
                Some(box_statement),
            ));
        }

        //------------------------------------
        pt::Statement::Try(_, _, option_paramlist_box_statement, _) => {
            if option_paramlist_box_statement.is_some() {
                let (param_list, box_statement) = option_paramlist_box_statement.unwrap();

                matches.append(&mut extract_target_statements_from_option_box_statement(
                    targets,
                    Some(box_statement),
                ));
            }
        }

        _ => (),
    }

    matches
}

pub fn extract_target_statements_from_option_box_statement(
    targets: HashSet<StatementTarget>,
    option_box_statement: Option<Box<pt::Statement>>,
) -> Vec<pt::Statement> {
    if option_box_statement.is_some() {
        let box_statement = option_box_statement.unwrap();
        return extract_target_statements_from_statement(targets, *box_statement);
    }

    vec![]
}

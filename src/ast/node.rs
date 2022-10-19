use super::ast::*;
use solang_parser::pt;

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
    pub fn contract_part(self) -> Option<pt::ContractPart> {
        match self {
            Self::ContractPart(contract_part) => Some(contract_part),
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone)]
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

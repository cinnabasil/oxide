use std::collections::HashMap;

pub type Ast = Vec<TopLevelItem>;

pub type FunctionParameters = HashMap<String, Type>;

#[derive(Debug)]
pub enum Type {
    I32
}

// bool = can_error (`!`)
pub type ReturnType = (Type, bool);

#[derive(Debug)]
pub enum TopLevelItem {
    Function(Function)
}

#[derive(Debug)]
pub struct Function {
    pub impure: bool,
    pub name: String,
    pub parameters: Option<FunctionParameters>,
    pub return_type: Option<ReturnType>,
    pub block: Option<Block>
}

pub type Block = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    Expression(Expression)
}

#[derive(Debug)]
pub enum LiteralType {
    Integer(isize),
    Float(f64),
    String(String),
    True,
    False
}

pub type CallParameters = Vec<Expression>;

#[derive(Debug)]
pub enum Expression {
    Literal(LiteralType),
    FunctionCall {
        name: String,
        parameters: Option<CallParameters>
    },
    MethodCall {
        path: Box<Expression>,
        name: String,
        parameters: Option<CallParameters>
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>
    },
    Unary,
    
    IfExpression {
        condition: Box<Expression>,
        block: Block
        // TODO: Else
    }
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Star,
    Divide,
    And,
    Or
}

#[derive(Debug)]
pub enum UnaryOperator {
    // -
    Negate,
    // !
    Not
}

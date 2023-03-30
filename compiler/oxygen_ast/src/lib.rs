use std::collections::HashMap;

pub type Ast = Vec<TopLevelItem>;

pub type FunctionParameters = HashMap<String, Type>;

pub enum Type {
    I32
}

// bool = can_error (`!`)
pub type ReturnType = (Type, bool);

pub enum TopLevelItem {
    Function {
        impure: bool,
        name: String,
        parameters: Option<FunctionParameters>,
        return_type: Option<ReturnType>,
        block: Option<Block>
    }    
}

pub type Block = Vec<Statement>;

pub enum Statement {
    Expression(Expression)
}

pub enum LiteralType {
    Integer(isize),
    Float(f64),
    String(String),
    True,
    False
}

type CallParameters = Vec<Expression>;

pub enum Expression {
    Literal(LiteralType),
    FunctionCall {
        name: String,
        parameters: CallParameters
    },
    MethodCall {
        path: Box<Expression>,
        name: String,
        parameters: CallParameters
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

pub enum BinaryOperator {
    Plus,
    Minus,
    Star,
    Divide,
    And,
    Or
}

pub enum UnaryOperator {
    // -
    Negate,
    // !
    Not
}

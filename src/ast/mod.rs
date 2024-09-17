use std::fmt;

pub type BlockStatement = Vec<Statement>;
pub type Program = BlockStatement;

#[derive(PartialEq, Clone, Debug)]
pub struct Identifier(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
  Forge(Identifier, Expression),
  Ignite(Expression),
  Expression(Expression),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
  Identifier(Identifier),
  Literal(Literal),
  Prefix(Prefix, Box<Expression>),
  Infix(Infix, Box<Expression>, Box<Expression>),
  If {
    cond: Box<Expression>,
    consequence: Box<Statement>,
    alternative: Option<BlockStatement>,
  },
  Function {
    parameters: Vec<Identifier>,
    body: BlockStatement,
  },
  Call {
    function: Box<Expression>,
    arguments: Vec<Expression>,
  },
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
  Integer(i64),
  String(String),
  Boolean(bool),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Prefix {
  Plus,
  Minus,
  Not,
}

impl fmt::Display for Prefix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Prefix::Plus => write!(f, "+"),
      Prefix::Minus => write!(f, "-"),
      Prefix::Not => write!(f, "!"),
    }
  }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Infix {
  Plus,
  Minus,
  Multiply,
  Divide,
  Equal,
  NotEqual,
  GreaterThan,
  LessThan,
}

impl fmt::Display for Infix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Infix::Plus => write!(f, "+"),
      Infix::Minus => write!(f, "-"),
      Infix::Divide => write!(f, "/"),
      Infix::Multiply => write!(f, "*"),
      Infix::Equal => write!(f, "=="),
      Infix::NotEqual => write!(f, "!="),
      Infix::GreaterThanEqual => write!(f, ">="),
      Infix::GreaterThan => write!(f, ">"),
      Infix::LessThanEqual => write!(f, "<="),
      Infix::LessThan => write!(f, "<"),
    }
  }
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
  Lowest,
  Equals,
  LessGreater,
  Sum,
  Product,
  Prefix,
  Call,
  Index,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Illegal,
  Eof,

  // Identifiers + literals
  Ident(String),
  Int(i64),
  String(String),
  Boolean(bool),

  // Statements
  Assign,
  If,
  Else,

  // Operators
  Plus,
  Minus,
  Bang,
  Asterisk,
  Slash,

  Equal,
  NotEqual,
  Lt,
  LessThanEqual,
  Gt,
  GreaterThanEqual,

  // Delimiters
  Comma,
  Colon,
  Semicolon,
  Lparen,
  Rparen,
  Lbrace,
  Rbrace,
  Lbracket,
  Rbracket,

  // Reseved keywords
  Weave,
  Forge,
  Ignite,
}

#[derive(Debug,PartialEq)]
pub enum AsaErrorKind {
  UndefinedFunction,
  VariableNotDefined(String),
  DivisionByZero,
  NumberOverflow,
  NumberUnderflow,
  TypeMismatch,
  Generic(String),  
  IoError(String),
}
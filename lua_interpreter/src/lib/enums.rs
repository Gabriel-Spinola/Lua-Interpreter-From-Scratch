use std::fmt;

use crate::vm::ExeState;

/// Ensure that the enum does not exceed 32bits.
#[derive(Debug)]
pub enum ByteCode {
  /// * 1st u8 - Index of the target stack;
  /// * 2nd u8 - Index of the global variable name;
  GetGlobal(u8, u8),

  /// Load costants onto the stack
  /// * 1st u8 - Index of the target stack;
  /// * 2nd u8 - Index of the constant in the constants table;
  LoadConst(u8, u8),

  /// * 1st u8 - Function position;
  /// * 2nd u8 - Number of parameters;
  Call(u8, u8),

  LoadNil(u8),
  LoadBool(u8, bool),
  LoadInteger(u8, i16),
}

/// Represents Lua values and Contains Lua type definition 
#[derive(Clone)]
pub enum Value {
  Nil,

  String(String),
  Boolean(bool),
  Float(f64),
  Integer(i64),

  Function(fn (&mut ExeState) -> i32)
}

impl fmt::Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Nil => write!(f, "nil"),
      Value::Boolean(b) => write!(f, "{b}"),
      Value::Integer(i) => write!(f, "{i}"),
      Value::Float(n) => write!(f, "{n:?}"),
      Value::String(s) => write!(f, "{s}"),
      Value::Function(_) => write!(f, "function"),
    }
  }
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    // TODO compare Integer vs Float
    match (self, other) {
      (Value::Nil, Value::Nil) => true,
      (Value::Boolean(b1), Value::Boolean(b2)) => *b1 == *b2,
      (Value::Integer(i1), Value::Integer(i2)) => *i1 == *i2,
      (Value::Float(f1), Value::Float(f2)) => *f1 == *f2,
      (Value::String(s1), Value::String(s2)) => *s1 == *s2,
      (Value::Function(f1), Value::Function(f2)) => std::ptr::eq(f1, f2),
      (_, _) => false,
    }
  }
}

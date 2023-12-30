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
}

/// Represents Lua values and Contains Lua type definition 
#[derive(Clone)]
pub enum Value {
  Nil,
  String(String),
  Function(fn (&mut ExeState) -> i32)
}

impl fmt::Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Nil => write!(f, "Nil"),
      Self::String(s) => write!(f, "{s}"),
      Self::Function(_) => write!(f, "Function"),
    }
  }
}

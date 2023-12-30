use std::fs::File;

use crate::{lib::enums::{Value, ByteCode}, lexer::{Lexer, Token}};

pub struct ParsedProto {
  pub constants: Vec<Value>,
  pub byte_codes: Vec<ByteCode>,
}

pub fn load(input: File) -> ParsedProto {
  let mut constants = Vec::new();
  let mut byte_codes = Vec::new();

  let mut lexer = Lexer::new(input);

  loop {
    match lexer.next() {
      // NOTE - `Name LiteralString` as function call
      Token::Name(name) => { 
        constants.push(Value::String(name));

        // NOTE - Since we currently only support function calls, the stack is only used for that
        // so the function must be at index 0.
        byte_codes.push(ByteCode::GetGlobal(0, (constants.len() - 1) as u8)); 

        if let Token::String(string) = lexer.next() {
          constants.push(Value::String(string));

          byte_codes.push(ByteCode::LoadConst(1, (constants.len() - 1) as u8));
          byte_codes.push(ByteCode::Call(0, 1));
        } else {
          panic!("Expected String");
        }
      }
      Token::EoF => break,
      token => panic!("Unexpect token: {token:?}"),
    }
  }

  dbg!(&constants);
  dbg!(&byte_codes);

  return ParsedProto { constants, byte_codes }
}

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
      Token::VarName(name) => { 
        let stored_constant = add_constant(
          &mut constants, Value::String(name)
        );

        // NOTE - Since we currently only support function calls, the stack is 
        // only used for that so the function must be at index 0.
        byte_codes.push(ByteCode::LoadConst(0, stored_constant as u8));   
        byte_codes.push(ByteCode::GetGlobal(0, stored_constant as u8));      

        match lexer.next() {
          Token::ParL => { // (
            let code = match lexer.next() {
              Token::Nil => ByteCode::LoadNil(1),
              Token::True => ByteCode::LoadBool(1, true),
              Token::False => ByteCode::LoadBool(1, false),
              Token::Float(float) => ByteCode::LoadConst(1, add_constant(&mut constants, Value::Float(float)) as u8),
              Token::String(string) => ByteCode::LoadConst(1, add_constant(&mut constants, Value::String(string)) as u8),
              
              Token::Integer(integer) => {
                if let Ok(int) = i16::try_from(integer) {
                  ByteCode::LoadInteger(1, int)
                } else {
                  ByteCode::LoadConst(1, add_constant(&mut constants, Value::Integer(integer)) as u8)
                }
              }

              _ => panic!("Invalid argument"),
            };

            byte_codes.push(code);

            if lexer.next() != Token::ParR {
              panic!("Expected ')'");
            }
          }

          Token::String(string) => {
            let code = ByteCode::LoadConst(1, add_constant(&mut constants, Value::String(string)) as u8);
            byte_codes.push(code);
          }

          _ => panic!("Expected string"),
        }

        byte_codes.push(ByteCode::Call(0, stored_constant as u8))
      }
      
      Token::EoF => break,
      token => panic!("Unexpect token: {token:?}"),
    }
  }

  dbg!(&constants);
  dbg!(&byte_codes);

  return ParsedProto { constants, byte_codes }
}

/// If the constant already exists return it otherwise add it into the vector 
/// and return its length.
fn add_constant(constants: &mut Vec<Value>, value: Value) -> usize {
  return constants
    .iter()
    .position(|constant| constant == &value)
    .unwrap_or_else(| | { constants.push(value); return constants.len() - 1 })
}

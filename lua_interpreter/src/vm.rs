use std::collections::HashMap;

use crate::{lib::enums::{Value, ByteCode}, parser::ParsedProto};

pub struct ExeState {
  globals: HashMap<String, Value>,
  stack: Vec<Value>
}

impl ExeState {
  pub fn new() -> Self {
    let mut globals = HashMap::new();
    globals.insert(String::from("print"), Value::Function(lib_print));

    return ExeState {
      globals,
      stack: Vec::new(),
    }
  }

  pub fn execute(&mut self, proto: &ParsedProto) {
    for code in proto.byte_codes.iter() {
      match *code {
        ByteCode::GetGlobal(index, name) => {
          let name = &proto.constants[name as usize];

          if let Value::String(key) = name {
            let value = self.globals.get(key)
              .unwrap_or(&Value::Nil)
              .clone();

            self.set_stack(index, value)
          }  
          else {
            panic!("Invalid Global Key: {name:?}");
          }
        }

        ByteCode::LoadConst(index, constant) => {
          let value = proto.constants[constant as usize].clone();

          self.set_stack(index, value)
        }

        ByteCode::Call(func, _) => {
          let func = &self.stack[func as usize];

          if let Value::Function(function) = func {
            function(self);
          } 
          else {
            panic!("Invalid function: {func:?}");
          }
        }

        ByteCode::LoadNil(index) => self.set_stack(index, Value::Nil),
        ByteCode::LoadBool(index, value) => self.set_stack(index, Value::Boolean(value)),
        ByteCode::LoadInteger(index, value) => self.set_stack(index, Value::Integer(value as i64)),
      }
    }
  }

  //  NOTE - stack setting behaviour
  fn set_stack(&mut self, index: u8, value: Value) {
    let index = index as usize;

    if index == self.stack.len() {
      self.stack.push(value);
    }
    else if index < self.stack.len() {
      self.stack[index] = value;
    }
    else {
      panic!("Fail to set_stack");
    }
  }
}

fn lib_print(state: &mut ExeState) -> i32 {
  // NOTE - We assume that the parameter is at the position 1 of the stack
  println!("{:?}", state.stack[1]);

  return 0;
}
mod lexer;
mod parser;
mod lib;
mod vm;

use std::env;
use std::fs::File;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    println!("Usage: {} script", args[0]);

    return;
  }

  let file = File::open(&args[1]).unwrap();
  let proto = parser::load(file);
  
  vm::ExeState::new().execute(&proto);
}

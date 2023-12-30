/// Ensure that the enum does'nt exceed 32bits.
#[derive(Debug)]
pub enum ByteCode {
  GetCall(u8, u8),
  LoadConst(u8, u8),
  Call(u8, u8),
}

fn main() {
  println!("Hello, world!");
}

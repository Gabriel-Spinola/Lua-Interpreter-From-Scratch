use std::{fs::File, io::{Read, Seek, SeekFrom}};

#[derive(Debug)]
pub enum Token {
  Name(String),
  String(String),
  EoF,
}

#[derive(Debug)]
pub struct Lexer {
  input: File,
}

impl Lexer {
  /// Creates a parser based on the input file.
  pub fn new(input: File) -> Self { 
    return Lexer { input };
  }

  /// Return the next token.
  pub fn next(&mut self) -> Token { 
    // NOTE - Reads the characters from the input file
    // if there's no characters left return '\0' for end of file.
    let character = self.read_characters();
    
    match character {
      // NOTE - Handle literal String
      '"' => {
        let mut string = String::new();

        loop {
          match self.read_characters() {
            '\0' => panic!("Unfineshed literal string"),
            '"' => break,
            character => string.push(character),
          }
        }

        return Token::String(string);
      }

      // NOTE - Handle names
      'A'..='Z' | 'a'..='z' | '_' => { 
        let mut name = String::new();
        name.push(character);

        loop {
          match self.read_characters() {
            '\0' => break,
            '_' => name.push('_'),
            
            character 
              if character.is_alphanumeric() => name.push(character),

            _ => {
              // This is done because the current character doesn't match any of the expected patterns,
              // so it needs to be "unread" or "reverted."
              self.input.seek(SeekFrom::Current(-1)).unwrap();

              break;
            }
          }
        }

        return Token::Name(name);
      }
      
      ' ' | '\n' | '\r' | '\t' => self.next(), // ignore blank spaces
      '\0' => Token::EoF,
      _ => panic!("Unexpected Character: {character}"),
    }
  }

  fn read_characters(&mut self) -> char {
    let mut buffer: [u8; 1] = [0];

    if self.input.read(&mut buffer).unwrap() == 1 {
      return buffer[0] as char
    } 
      
    return '\0' 
  }
}

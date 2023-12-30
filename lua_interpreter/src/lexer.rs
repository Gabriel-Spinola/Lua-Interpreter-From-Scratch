use std::{fs::File, io::{Read, Seek, SeekFrom}};

#[derive(Debug)]
pub enum Token {
  // Keywords
  And,    Break,  Do,     Else,   Elseif, End,
  False,  For,    Function, Goto, If,     In,
  Local,  Nil,    Not,    Or,     Repeat, Return,
  Then,   True,   Until,  While,
  
  // +      -      *       /       %       ^       #
  Add,    Sub,    Mul,    Div,    Mod,    Pow,    Len,
  // &       ~       |       <<      >>      //
  BitAnd, BitXor, BitOr,  ShiftL, ShiftR, Idiv,
  // ==     ~=     <=      >=      <         >        =
  Equal,  NotEq,  LesEq,  GreEq,  Less,   Greater, Assign,
  // (       )       {       }       [       ]       ::
  ParL,   ParR,   CurlyL, CurlyR, SqurL,  SqurR,  DoubColon,
  // ;              :       ,       .       ..    ...
  SemiColon,      Colon,  Comma,  Dot,    Concat, Dots,
  
  VarName(String),
  
  // Constant Values
  Integer(i64),
  Float(f64),
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
    let character = self.read_character();
    
    match character {
      // ANCHOR - Assignment and Comparison Operators
      '+' => Token::Add,
      '*' => Token::Mul,
      '%' => Token::Mod,
      '^' => Token::Pow,
      '#' => Token::Len,
      '&' => Token::BitAnd,
      '|' => Token::BitOr,

      '-' => {
        if self.read_character() == '-' {
          self.read_comment();

          return self.next()
        }
        else {
          self.putback_char();

          return  Token::Sub;
        }
      },

      '/' => {
        if self.read_character() == '/' {
          return Token::Idiv;
        }

        self.putback_char();
        return Token::Div;
      },

      '<' => {
        if self.read_character() == '<' {
          return Token::ShiftL;
        }

        if self.read_character() == '=' {
          return Token::LesEq;
        }

        self.putback_char();
        return Token::Less;
      },

      '>' => {
        if self.read_character() == '>' {
          return Token::ShiftR;
        }

        if self.read_character() == '=' {
          return Token::GreEq;
        }

        self.putback_char();
        return Token::Greater;
      },

      '=' => {
        if self.read_character() == '=' {
          return Token::Equal;
        }

        self.putback_char();
        return Token::Assign;
      },

      '~' => {
        if self.read_character() == '=' {
          return Token::NotEq
        }

        self.putback_char();
        return Token::BitXor
      },

      // ANCHOR - Punctuation and Other Operators
      '(' => Token::ParL,
      ')' => Token::ParR,
      '{' => Token::CurlyL,
      '}' => Token::CurlyR,
      '[' => Token::SqurL,
      ']' => Token::SqurR,
      ';' => Token::SemiColon,
      ',' => Token::Comma,

      ':' => {
        if self.read_character() == ':' {
          return Token::DoubColon;
        }

        self.putback_char();
        return Token::Colon
      },

      '.' => {
        if self.read_character() == '.' {
          if self.read_character() == '.' {
            return Token::Dots;
          }

          self.putback_char();
          return Token::Concat;
        }

        self.putback_char();
        return Token::Dot;
      }

      '"' | '\'' => self.read_literal_strings(character),
      'A'..='Z' | 'a'..='z' | '_' => self.read_var_names(character),
      '0'..='9' => self.read_numbers(character), 

      ' ' | '\n' | '\r' | '\t' => self.next(), // ignore blank spaces
      '\0' => Token::EoF,

      _ => panic!("Unexpected Character: {character}"),
    }
  }

  fn read_character(&mut self) -> char {
    let mut buffer: [u8; 1] = [0];

    if self.input.read(&mut buffer).unwrap() == 1 {
      return buffer[0] as char
    } 
      
    return '\0' 
  }

  /// This is done because the current character doesn't match any of the expected patterns,
  /// so it needs to be "unread" or "reverted".
  fn putback_char(&mut self) {
    self.input.seek(SeekFrom::Current(-1)).unwrap();
  }

  fn read_literal_strings(&mut self, quote: char) -> Token {
    let mut string = String::new();

    loop {
      match self.read_character() {
        '\0' => panic!("Unfineshed literal string"),

        // NOTE - Check if the quote (' | ") is being closed
        character if character == quote => break,
        character => string.push(character),
      }
    }

    return Token::String(string);
  }

  fn read_var_names(&mut self, first_char: char) -> Token {
    let mut name = first_char.to_string();

    loop {
      let char = self.read_character();

      if char.is_alphanumeric() || char == '_' {
        name.push(char);
      } 
      else {
        self.putback_char();

        break;
      }
    }

    return match &name as &str {
      // ANCHOR - Keywords
      "and"      => Token::And,
      "break"    => Token::Break,
      "do"       => Token::Do,
      "else"     => Token::Else,
      "elseif"   => Token::Elseif,
      "end"      => Token::End,
      "false"    => Token::False,
      "for"      => Token::For,
      "function" => Token::Function,
      "goto"     => Token::Goto,
      "if"       => Token::If,
      "in"       => Token::In,
      "local"    => Token::Local,
      "nil"      => Token::Nil,
      "not"      => Token::Not,
      "or"       => Token::Or,
      "repeat"   => Token::Repeat,
      "return"   => Token::Return,
      "then"     => Token::Then,
      "true"     => Token::True,
      "until"    => Token::Until,
      "while"    => Token::While,

      // ANCHOR - Variable names 
      _          => Token::VarName(name),
    }
  }


  fn read_numbers(&mut self, first: char) -> Token {
    // heximal
    if first == '0' {
      let second = self.read_character();

      if second == 'x' || second == 'X' {
        todo!()
      }

      self.putback_char();
    }

    // decimal
    let mut number = char::to_digit(first, 10).unwrap() as i64;
    
    loop {
      let char = self.read_character();

      if let Some(d) = char::to_digit(char, 10) {
        number = number * 10 + d as i64;
      } 
      
      if char == '.' {
        return self.read_number_fraction(number)
      } 
      
      if char == 'e' || char == 'E' {
        todo!()
      } 

      self.putback_char();
      
      break;   
    }

    // check following
    let fch = self.read_character();

    if fch.is_alphabetic() || fch == '.' {
      panic!("malformat number");
    } else {
      self.putback_char();
    }

    Token::Integer(number)
  }

  fn read_number_fraction(&mut self, i: i64) -> Token {
    let mut n: i64 = 0;
    let mut x: f64 = 1.0;

    loop {
        let ch = self.read_character();

        if let Some(d) = char::to_digit(ch, 10) {
          n = n * 10 + d as i64;
          x *= 10.0;
        } else {
          self.putback_char();
          break;
        }
    }

    Token::Float(i as f64 + n as f64 / x)
  }

  fn read_comment(&mut self) {
    match self.read_character() {
      '[' => todo!("long comment"),

      _ => { // line comment
        loop {
          let char = self.read_character();

          if char == '\n' || char == '\0' {
            break;
          }
        }
      }
    }
  }
}

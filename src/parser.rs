use chrono::Local;
use regex::Regex;

pub struct Parser {
  counter: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ErrorCode {
  // Ok = 0,
  InvalidCharacter,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Error {
  pub code: ErrorCode,
  pub column: usize,
  pub message: String,
}

impl Parser {
  pub fn new() -> Self {
    Parser { counter: 1 }
  }

  pub fn parse(self, name: &str) -> Result<String, Error> {
    let mut question_count = 0;
    let mut question_position = 0;
    let mut backslash_flag = false;

    let mut ret: Vec<char> = vec![];
    for (i, c) in name.chars().enumerate() {
      println!("{}: {}", i, c);

      if c == '\\' {
        backslash_flag = true;
        continue;
      }

      if c == '?' {
        if question_count <= 0 {
          question_position = i;
        }
        question_count += 1;
        continue;
      } else {
        if question_count > 0 {
          println!("{:01$}", self.counter, question_count);
        }
        question_count = 0;
      }

      if backslash_flag {
        backslash_flag = false;

        match c {
          'Y' => {
            // Year
          }
          'm' => {
            // Month
          }
          'd' => {
            // Day
          }
          'H' => {
            // Hour
          }
          'M' => {
            // Minute
          }
          'S' => {
            // Second
          }
          _ => {
            return Err(Error {
              code: ErrorCode::InvalidCharacter,
              column: i,
              message: format!("Invalid character: {}", c),
            });
          }
        }
        continue;
      } else {
        ret.push(c);
      }
    }

    if question_count > 0 {
      let s = format!("{:01$}", self.counter, question_count);
      ret.extend_from_slice(s.chars().collect::<Vec<_>>().as_slice());
    }

    println!("ret: {}", ret.iter().collect::<String>());

    Ok(ret.iter().collect::<String>())
  }
}

#[cfg(test)]
mod tests {
  use crate::parser::Parser;

  #[test]
  fn test_parse() {
    let p = Parser::new();

    let name = "test";
    let r = p.parse(name);

    assert_eq!(String::from(name), r.unwrap());
  }

  #[test]
  fn test_parse_with_incremental_1() {
    let p = Parser::new();

    let name = "test?";
    let r = p.parse(name);

    assert_eq!(String::from("test1"), r.unwrap());
  }
  #[test]
  fn test_parse_with_incremental_2() {
    let p = Parser::new();

    let name = "test???";
    let r = p.parse(name);

    assert_eq!(String::from("test001"), r.unwrap());
  }
}

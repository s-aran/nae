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
    Parser { counter: 0 }
  }

  pub fn parse(&mut self, name: &str) -> Result<String, Error> {
    let mut question_count = 0;
    let mut begin_pos = 0;
    let mut backslash_flag = false;

    self.counter += 1;

    let mut ret: Vec<char> = vec![];
    for (i, c) in name.chars().enumerate() {
      println!("{}: {}", i, c);

      if c == '\\' {
        backslash_flag = true;
        begin_pos = i;
        continue;
      }

      if c == '?' {
        if question_count <= 0 {
          begin_pos = i;
        }

        question_count += 1;
        continue;
      } else {
        if question_count > 0 {
          println!("{:01$}", self.counter, question_count);
          let s = format!("{:01$}", self.counter, question_count);
          ret.extend(s.chars());
        }

        question_count = 0;
      }

      if backslash_flag {
        backslash_flag = false;

        let datetime = Local::now();

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
      ret.extend(s.chars());
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
    let mut p = Parser::new();

    let name = "test";
    let r = p.parse(name);

    assert_eq!(String::from(name), r.unwrap());
  }

  #[test]
  fn test_parse_with_incremental_1() {
    let mut p = Parser::new();

    let name = "test?";
    let r = p.parse(name);

    assert_eq!(String::from("test1"), r.unwrap());
  }

  #[test]
  fn test_parse_with_incremental_2() {
    let mut p = Parser::new();

    let name = "test???";
    let r = p.parse(name);

    assert_eq!(String::from("test001"), r.unwrap());
  }

  #[test]
  fn test_parse_with_incremental_3() {
    let mut p = Parser::new();

    let name = "test???123";
    let r = p.parse(name);

    assert_eq!(String::from("test001123"), r.unwrap());
  }

  #[test]
  fn test_parse_with_incremental_4() {
    let mut p = Parser::new();

    let name = "???test";
    let r = p.parse(name);

    assert_eq!(String::from("001test"), r.unwrap());
  }

  #[test]
  fn test_parse_with_incremental_5() {
    let mut p = Parser::new();

    let name = "???";
    let r = p.parse(name);

    assert_eq!(String::from("001"), r.unwrap());
  }

  #[test]
  fn test_parse_with_incremental_6() {
    let mut p = Parser::new();
    let name = "???";

    {
      let r = p.parse(name);
      assert_eq!(String::from("001"), r.unwrap());
    }

    {
      let r = p.parse(name);
      assert_eq!(String::from("002"), r.unwrap());
    }
  }
}

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
    let mut backslash_flag = false;

    self.counter += 1;

    let mut ret: Vec<char> = vec![];
    for (i, c) in name.chars().enumerate() {
      // println!("{}: {}", i, c);

      if c == '\\' {
        backslash_flag = true;
        continue;
      }

      if c == '?' {
        question_count += 1;
        continue;
      } else {
        if question_count > 0 {
          // println!("{:01$}", self.counter, question_count);
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
            // Year (four digit)
            let s = datetime.format("%Y").to_string();
            ret.extend(s.chars());
          }
          'y' => {
            // Year (two digit)
            let s = datetime.format("%y").to_string();
            ret.extend(s.chars());
          }
          'm' => {
            // Month
            let s = datetime.format("%m").to_string();
            ret.extend(s.chars());
          }
          'd' => {
            // Day
            let s = datetime.format("%d").to_string();
            ret.extend(s.chars());
          }
          'H' => {
            // Hour
            let s = datetime.format("%H").to_string();
            ret.extend(s.chars());
          }
          'M' => {
            // Minute
            let s = datetime.format("%M").to_string();
            ret.extend(s.chars());
          }
          'S' => {
            // Second
            let s = datetime.format("%S").to_string();
            ret.extend(s.chars());
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

    // println!("ret: {}", ret.iter().collect::<String>());

    Ok(ret.iter().collect::<String>())
  }
}

#[cfg(test)]
mod tests {
  use chrono::Local;

  use crate::parser::{Error, ErrorCode, Parser};

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

  #[test]
  fn test_parse_with_incremental_7() {
    let mut p = Parser::new();
    let name = "?";

    for i in 1..1000 + 1 {
      let r = p.parse(name);
      assert_eq!(format!("{}", i), r.unwrap());
    }
  }

  #[test]
  fn test_parse_with_datetime_4year_1() {
    let mut p = Parser::new();

    let name = "test\\Y";
    let r = p.parse(name);

    let now = Local::now();

    assert_eq!(now.format("test%Y").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_2year_1() {
    let mut p = Parser::new();

    let name = "test\\y";
    let r = p.parse(name);

    let now = Local::now();

    assert_eq!(now.format("test%y").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_month_1() {
    let mut p = Parser::new();

    let name = "test\\m";
    let r = p.parse(name);

    let now = Local::now();

    assert_eq!(now.format("test%m").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_day_1() {
    let mut p = Parser::new();

    let name = "test\\d";
    let r = p.parse(name);

    let now = Local::now();

    assert_eq!(now.format("test%d").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_hour_1() {
    let mut p = Parser::new();

    let name = "test\\H";
    let r = p.parse(name);

    let now = Local::now();

    assert_eq!(now.format("test%H").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_minute_1() {
    let mut p = Parser::new();

    let name = "test\\Y";
    let r = p.parse(name);

    let now = Local::now();

    assert_eq!(now.format("test%Y").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_second_1() {
    let mut p = Parser::new();

    let name = "test\\S";
    let r = p.parse(name);

    let now = Local::now();

    assert_eq!(now.format("test%S").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_full_1() {
    let mut p = Parser::new();

    let name = "test\\y - \\Y-\\m-\\d_\\H-\\M-\\S";
    let r = p.parse(name);

    let now = Local::now();

    assert_eq!(
      now.format("test%y - %Y-%m-%d_%H-%M-%S").to_string(),
      r.unwrap()
    );
  }

  #[test]
  fn test_parse_invalid_1() {
    let mut p = Parser::new();

    let name = "test\\Q";
    let r = p.parse(name);

    assert_eq!(
      Error {
        column: 5,
        code: ErrorCode::InvalidCharacter,
        message: "Invalid character: Q".to_string()
      },
      r.unwrap_err()
    )
  }

  #[test]
  fn test_parse_invalid_2() {
    let mut p = Parser::new();

    let name = "test\\Qtest";
    let r = p.parse(name);

    assert_eq!(
      Error {
        column: 5,
        code: ErrorCode::InvalidCharacter,
        message: "Invalid character: Q".to_string()
      },
      r.unwrap_err()
    )
  }

  #[test]
  fn test_parse_invalid_3() {
    let mut p = Parser::new();

    let name = "test\\Q\\W";
    let r = p.parse(name);

    assert_eq!(
      Error {
        column: 5,
        code: ErrorCode::InvalidCharacter,
        message: "Invalid character: Q".to_string()
      },
      r.unwrap_err()
    )
  }
  
  #[test]
  fn test_parse_invalid_4() {
    let mut p = Parser::new();

    let name = "\\Q";
    let r = p.parse(name);

    assert_eq!(
      Error {
        column: 1,
        code: ErrorCode::InvalidCharacter,
        message: "Invalid character: Q".to_string()
      },
      r.unwrap_err()
    )
  }
}

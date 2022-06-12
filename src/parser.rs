use chrono::Local;
use regex::Regex;

pub struct Parser {
  counter: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ErrorCode {
  // Ok = 0,
  NoOptionalData,
  InvalidCharacter,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Error {
  pub code: ErrorCode,
  pub column: usize,
  pub message: String,
}

pub struct OptionalData {
  pub file_name: String,
}

impl Parser {
  pub fn new() -> Self {
    Parser { counter: 0 }
  }

  pub fn parse(&mut self, name: &str, optinal: Option<&OptionalData>) -> Result<String, Error> {
    let mut question_count = 0;
    let mut backslash_flag = false;

    self.counter += 1;

    let mut ret: Vec<char> = vec![];
    for (i, c) in name.chars().enumerate() {
      // println!("{}: {}", i, c);

      if c == '\\' {
        if !backslash_flag {
          backslash_flag = true;
          continue;
        }
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
          '\\' => ret.push('\\'),
          '0' => match optinal {
            Some(opt) => ret.extend(opt.file_name.chars()),
            None => {
              return Err(Error {
                code: ErrorCode::NoOptionalData,
                column: i,
                message: "OptionalData not specified".to_string(),
              });
            }
          },
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
          'b' => {
            // Month name (abbreviated)
            let s = datetime.format("%b").to_string();
            ret.extend(s.chars());
          }
          'B' => {
            // Month name (full)
            let s = datetime.format("%B").to_string();
            ret.extend(s.chars());
          }
          'd' => {
            // Day
            let s = datetime.format("%d").to_string();
            ret.extend(s.chars());
          }
          'a' => {
            // Weekday name (abbreviated)
            let s = datetime.format("%a").to_string();
            ret.extend(s.chars());
          }
          'A' => {
            // Weekday name (full)
            let s = datetime.format("%A").to_string();
            ret.extend(s.chars());
          }
          'p' => {
            // am/pm (12-hour clock)
            let s = datetime.format("%p").to_string();
            ret.extend(s.chars());
          }
          'P' => {
            // AM/PM (12-hour clock)
            let s = datetime.format("%P").to_string();
            ret.extend(s.chars());
          }
          'H' => {
            // Hour (24-hour clock)
            let s = datetime.format("%H").to_string();
            ret.extend(s.chars());
          }
          'I' => {
            // Hour (12-hour clock)
            let s = datetime.format("%I").to_string();
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

  use crate::parser::{Error, ErrorCode, OptionalData, Parser};

  #[test]
  fn test_parse() {
    let mut p = Parser::new();

    let name = "%test%%";
    let r = p.parse(name, None);

    assert_eq!(String::from(name), r.unwrap());
  }

  #[test]
  fn test_parse_with_zero_1() {
    let mut p = Parser::new();
    let data = OptionalData {
      file_name: String::from("test.txt"),
    };

    let name = "test\\0";
    let r = p.parse(name, Some(&data));

    assert_eq!(String::from("testtest.txt"), r.unwrap());
  }

  #[test]
  fn test_parse_with_incremental_1() {
    let mut p = Parser::new();

    let name = "test?";
    let r = p.parse(name, None);

    assert_eq!(String::from("test1"), r.unwrap());
  }

  #[test]
  fn test_parse_with_incremental_2() {
    let mut p = Parser::new();

    let name = "test???";
    let r = p.parse(name, None);

    assert_eq!(String::from("test001"), r.unwrap());
  }

  #[test]
  fn test_parse_with_incremental_3() {
    let mut p = Parser::new();

    let name = "test???123";
    let r = p.parse(name, None);

    assert_eq!(String::from("test001123"), r.unwrap());
  }

  #[test]
  fn test_parse_with_incremental_4() {
    let mut p = Parser::new();

    let name = "???test";
    let r = p.parse(name, None);

    assert_eq!(String::from("001test"), r.unwrap());
  }

  #[test]
  fn test_parse_with_incremental_5() {
    let mut p = Parser::new();

    let name = "???";
    let r = p.parse(name, None);

    assert_eq!(String::from("001"), r.unwrap());
  }

  #[test]
  fn test_parse_with_incremental_6() {
    let mut p = Parser::new();
    let name = "???";

    {
      let r = p.parse(name, None);
      assert_eq!(String::from("001"), r.unwrap());
    }

    {
      let r = p.parse(name, None);
      assert_eq!(String::from("002"), r.unwrap());
    }
  }

  #[test]
  fn test_parse_with_incremental_7() {
    let mut p = Parser::new();
    let name = "?";

    for i in 1..1000 + 1 {
      let r = p.parse(name, None);
      assert_eq!(format!("{}", i), r.unwrap());
    }
  }

  #[test]
  fn test_parse_with_datetime_4year_1() {
    let mut p = Parser::new();

    let name = "test\\Y";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(now.format("test%Y").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_2year_1() {
    let mut p = Parser::new();

    let name = "test\\y";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(now.format("test%y").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_month_1() {
    let mut p = Parser::new();

    let name = "test\\m";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(now.format("test%m").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_month_name_abbreviated_1() {
    let mut p = Parser::new();

    let name = "test\\b";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(now.format("test%b").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_month_name_full_1() {
    let mut p = Parser::new();

    let name = "test\\B";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(now.format("test%B").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_day_1() {
    let mut p = Parser::new();

    let name = "test\\d";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(now.format("test%d").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_weekday_name_abbreviated_1() {
    let mut p = Parser::new();

    let name = "test\\a";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(now.format("test%a").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_weekday_name_full_1() {
    let mut p = Parser::new();

    let name = "test\\A";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(now.format("test%A").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_24hour_1() {
    let mut p = Parser::new();

    let name = "test\\H";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(now.format("test%H").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_12hour_1() {
    let mut p = Parser::new();

    let name = "test\\I";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(now.format("test%I").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_ampm_small_1() {
    let mut p = Parser::new();

    let name = "test\\p";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(now.format("test%p").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_ampm_large_1() {
    let mut p = Parser::new();

    let name = "test\\P";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(now.format("test%P").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_minute_1() {
    let mut p = Parser::new();

    let name = "test\\Y";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(now.format("test%Y").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_second_1() {
    let mut p = Parser::new();

    let name = "test\\S";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(now.format("test%S").to_string(), r.unwrap());
  }

  #[test]
  fn test_parse_with_datetime_full_1() {
    let mut p = Parser::new();

    let name = "test\\y - \\Y-\\m-\\d_\\H-\\M-\\S";
    let r = p.parse(name, None);

    let now = Local::now();

    assert_eq!(
      now.format("test%y - %Y-%m-%d_%H-%M-%S").to_string(),
      r.unwrap()
    );
  }

  #[test]
  fn test_parse_with_backslash_escape_1() {
    let mut p = Parser::new();

    let name = "test\\\\";
    let r = p.parse(name, None);

    assert_eq!(String::from("test\\"), r.unwrap());
  }

  #[test]
  fn test_parse_with_backslash_escape_2() {
    let mut p = Parser::new();

    let name = "test\\\\test";
    let r = p.parse(name, None);

    assert_eq!(String::from("test\\test"), r.unwrap());
  }

  #[test]
  fn test_parse_invalid_1() {
    let mut p = Parser::new();

    let name = "test\\Q";
    let r = p.parse(name, None);

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
    let r = p.parse(name, None);

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
    let r = p.parse(name, None);

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
    let r = p.parse(name, None);

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

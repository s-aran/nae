use std::io::{Error, ErrorKind};
use std::path::Path;
use crate::parser::Parser;
use crate::filesystem::FileSystem;

pub struct Renamer {
  parser: Parser,
}

impl Renamer {
  pub fn new() -> Self {
    Renamer {
      parser: Parser::new(),
    }
  }

  pub fn rename(&mut self, target: &Path, name_pattern: &str) -> std::io::Result<()> {
    match self.parser.parse(name_pattern, None) {
      Ok(name) => {
        let new_name = target.with_file_name(name);
        FileSystem::rename(target, &new_name.to_str().unwrap())?;
        Ok(())
      }
      Err(e) => {
        println!("{}", e.message);
        Err(Error::new(ErrorKind::InvalidInput, e.message))
      }
    }
  }

  pub fn rename_dry_run(&mut self, target: &Path, name_pattern: &str) -> std::io::Result<()> {
    match self.parser.parse(name_pattern, None) {
      Ok(name) => {
        let new_name = target.with_file_name(name);
        println!("{}", new_name.to_str().unwrap());
        Ok(())
      }
      Err(e) => {
        println!("{}", e.message);
        Err(Error::new(ErrorKind::InvalidInput, e.message))
      }
    }
  }
}

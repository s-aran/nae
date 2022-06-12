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
    let new_name = self.parser.parse(name_pattern, None);
    FileSystem::rename(target, name_pattern)
  }
}

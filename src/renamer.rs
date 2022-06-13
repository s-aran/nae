use crate::filesystem::FileSystem;
use crate::parser::{OptionalData, Parser};
use std::io::{Error, ErrorKind};
use std::path::Path;

pub struct Renamer {
  parser: Parser,
}

impl Renamer {
  pub fn new() -> Self {
    Renamer {
      parser: Parser::new(),
    }
  }

  ///
  /// Rename file to the new name.
  ///
  /// # Arguments
  /// * `path` - path to file
  /// * `new_name` - new name
  /// * `dry_run` - dry run
  ///
  /// # Return
  /// * `std::io::Result<String>` - new name
  ///
  /// # Example
  /// ```
  /// use nae::renamer::Renamer;
  /// use std::path::Path;
  /// 
  /// let mut renamer = Renamer::new();
  /// let path = Path::new("README.md");
  /// let new_name = renamer.rename(&path, "new_name", false).unwrap();
  /// assert_eq!(new_name, "new_name");
  /// ```
  ///
  pub fn rename(
    &mut self,
    target: &Path,
    name_pattern: &str,
    dry_run: bool,
  ) -> std::io::Result<String> {
    let optional_data = OptionalData {
      file_name: target.file_name().unwrap().to_str().unwrap().to_string(),
    };
    match self.parser.parse(name_pattern, Some(&optional_data)) {
      Ok(name) => {
        let new_name = target.with_file_name(name);
        if !dry_run {
          FileSystem::rename(target, &new_name.to_str().unwrap())?;
        }
        Ok(new_name.to_str().unwrap().to_string())
      }
      Err(e) => {
        println!("{}", e.message);
        Err(Error::new(ErrorKind::InvalidInput, e.message))
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use std::{fs::File, io::Write, path::Path};

  fn create_file(path: &Path) -> bool {
    let mut file = match File::create(path) {
      Ok(file) => file,
      Err(why) => {
        println!("{}", why);
        return false;
      }
    };

    match file.write_all(b"\n") {
      Ok(_) => {
        return true;
      }
      Err(why) => {
        println!("{}", why);
        return false;
      }
    };
  }

  fn delete_file(path: &Path) -> bool {
    match std::fs::remove_file(path) {
      Ok(_) => {
        return true;
      }
      Err(why) => {
        println!("{}", why);
        return false;
      }
    };
  }

  fn exists_file(path: &Path) -> bool {
    path.exists()
  }

  #[test]
  fn test_rename_1() {
    let mut renamer = super::Renamer::new();
    let target = Path::new("test_renamer_1.txt");
    let name_pattern = "test_renamer_1\\0.txt";
    let dry_run = false;
    let expected_name = Path::new("test_renamer_1test_renamer_1.txt.txt");

    assert!(create_file(target));
    assert!(exists_file(target));
    assert!(
      renamer.rename(target, name_pattern, dry_run).ok().unwrap()
        == expected_name.to_str().unwrap()
    );
    assert!(!exists_file(target));
    assert!(exists_file(expected_name));
    assert!(delete_file(expected_name));
    assert!(!exists_file(expected_name));
  }

  #[test]
  fn test_rename_dry_run_1() {
    let mut renamer = super::Renamer::new();
    let target = Path::new("test_renamer_2.txt");
    let name_pattern = "test_renamer_2\\0.txt";
    let dry_run = true;
    let expected_name = Path::new("test_renamer_2test_renamer_2.txt.txt");

    assert!(create_file(target));
    assert!(exists_file(target));
    assert!(
      renamer.rename(target, name_pattern, dry_run).ok().unwrap()
        == expected_name.to_str().unwrap()
    );
    assert!(exists_file(target));
    assert!(!exists_file(expected_name));
    assert!(delete_file(target));
    assert!(!exists_file(target));
  }
}

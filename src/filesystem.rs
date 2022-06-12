use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub struct FileSystem {}

impl FileSystem {
  pub fn rename(target: &Path, new_name: &str) -> std::io::Result<()> {
    if target.file_name() == None {
      return Err(Error::new(ErrorKind::InvalidInput, "Invalid filename"));
    }

    // let new_path = target.with_file_name(name);
    match std::fs::rename(target, new_name) {
      Ok(_) => Ok(()),
      Err(e) => {
        // println!("rename: {}", e);
        Err(e)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::filesystem::FileSystem;
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
  fn test_create_and_delete_file() {
    let test_file_path = Path::new("test1.txt");
    assert!(create_file(test_file_path));
    assert!(exists_file(test_file_path));
    assert!(delete_file(test_file_path));
    assert!(!exists_file(test_file_path));
  }

  #[test]
  fn test_rename_1() {
    let test_file_path = Path::new("test2.txt");
    let renamed_file_path = Path::new("test3.txt");
    
    assert!(create_file(test_file_path));
    assert!(exists_file(test_file_path));
    assert!(FileSystem::rename(test_file_path, renamed_file_path.to_str().unwrap()).is_ok());
    assert!(!exists_file(test_file_path));
    assert!(exists_file(renamed_file_path));
    assert!(delete_file(renamed_file_path));
    assert!(!exists_file(renamed_file_path));
  }
}

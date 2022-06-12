use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub struct FileSystem {}

impl FileSystem {
  pub fn rename(target: &Path, new_name: &str) -> std::io::Result<()> {
    if target.file_name() == None {
      return Err(Error::new(ErrorKind::InvalidInput, "Invalid filename"));
    }

    match std::fs::rename(target, new_name) {
      Ok(_) => Ok(()),
      Err(e) => {
        // println!("rename: {}", e);
        Err(e)
      }
    }
  }

  ///
  /// Enumerate files in the target directory.
  /// If the target is a file, returns an error.
  ///
  /// # Arguments
  /// * `target` - The target directory.
  /// * `recursive` - If true, enumerate files recursively.
  /// * `callback` - The callback function.
  /// 
  /// # Returns
  /// * `Ok(())` - If the operation succeeded.
  /// * `Err(e)` - If the operation failed.
  /// 
  /// # Examples
  /// ```
  /// use std::path::Path;
  /// use nae::filesystem::FileSystem;
  /// 
  /// let target = Path::new("./");
  /// let mut callback = |path: &Path| {
  ///  println!("{}", path.to_str().unwrap());    // Prints the file name. (e.g. "./README.md")
  /// };
  /// 
  /// FileSystem::enum_files(&target, false, &mut callback).unwrap();
  /// ```
  /// 
  pub fn enum_files(
    path: &Path,
    recursive: bool,
    callback: &mut dyn FnMut(&Path),
  ) -> std::io::Result<()> {
    if !path.is_dir() {
      return Err(Error::new(ErrorKind::InvalidInput, "Invalid path"));
    }

    let files = path.read_dir().unwrap();

    for dir_entry in files {
      let dir_entry = dir_entry.unwrap();
      let path = dir_entry.path();

      if recursive && path.is_dir() {
        FileSystem::enum_files(&path, recursive, callback)?;
        return Ok(());
      } else {
        println!("{}", path.to_str().unwrap());
        callback(&path);
      }
    }

    Ok(())
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

  #[test]
  fn test_enum_files_1() {
    let dir = ".";
    let test_file_path = Path::new("test4.txt");

    let mut called = false;
    let mut test_file_found = false;

    let mut func = |path: &Path| {
      println!("{}", path.to_str().unwrap());
      called = true;
      if path == Path::new(dir).join(test_file_path) {
        test_file_found = true;
      }
    };

    assert!(create_file(test_file_path));
    assert!(exists_file(test_file_path));

    assert!(FileSystem::enum_files(Path::new(dir), false, &mut func).is_ok());

    assert!(called);
    assert!(test_file_found);

    assert!(delete_file(test_file_path));
    assert!(!exists_file(test_file_path));
  }

  #[test]
  fn test_enum_files_2() {
    let invalid_dir = "README.md";

    let mut called = false;

    let mut func = |path: &Path| {
      println!("{}", path.to_str().unwrap());
      called = true;
    };

    assert!(exists_file(Path::new(invalid_dir)));
    assert!(FileSystem::enum_files(Path::new(invalid_dir), false, &mut func).is_err());
    assert!(!called);
  }
}

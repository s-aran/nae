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

  #[cfg(target_os = "windows")]
  pub fn get_id_by_filename(path: &Path) -> Result<String, String> {
    use std::{
      mem::size_of_val,
      os::{raw::c_void, windows::prelude::OsStrExt},
    };

    use windows_sys::Win32::{
      Foundation::{CloseHandle, GetLastError, HANDLE, INVALID_HANDLE_VALUE},
      Storage::FileSystem::{
        CreateFileW, FileIdInfo, GetFileInformationByHandleEx, FILE_ATTRIBUTE_NORMAL,
        FILE_GENERIC_READ, FILE_ID_128, FILE_ID_INFO, FILE_SHARE_READ, OPEN_EXISTING,
      },
    };

    let mut wstr: Vec<u16> = path.as_os_str().encode_wide().collect();
    // terminal character
    wstr.push(0x0000);

    let h_file: HANDLE = unsafe {
      CreateFileW(
        wstr.as_ptr(),
        FILE_GENERIC_READ,
        FILE_SHARE_READ,
        std::ptr::null(),
        OPEN_EXISTING,
        FILE_ATTRIBUTE_NORMAL,
        0,
      )
    };

    if h_file == INVALID_HANDLE_VALUE {
      return Err("file open failed.".to_string());
    }

    let mut id: FILE_ID_INFO = FILE_ID_INFO {
      VolumeSerialNumber: 0,
      FileId: FILE_ID_128 {
        Identifier: Default::default(),
      },
    };

    // GetFileInformationByHandleEx() returned FALSE(=0)?
    if unsafe {
      GetFileInformationByHandleEx(
        h_file,
        FileIdInfo,
        &mut id as *mut _ as *mut c_void,
        size_of_val(&id).try_into().unwrap(),
      )
    } == 0
    {
      return Err(format!(
        "GetFileInformationByHandleEx() failed. code: {}",
        unsafe { GetLastError() }
      ));
    }

    unsafe { CloseHandle(h_file) };

    // [u8; 16] -> String
    let mut id_str = String::new();
    for i in id.FileId.Identifier.iter().rev() {
      id_str += format!("{:02X}", i).as_str();
    }

    Ok(id_str)
  }

  #[cfg(target_os = "windows")]
  pub fn get_filename_by_id(id_str: &String, hint_dir: &Path) -> Result<String, String> {
    use std::{mem, os::windows::prelude::OsStrExt};

    use windows_sys::Win32::{
      Foundation::{CloseHandle, GetLastError, HANDLE, INVALID_HANDLE_VALUE, NO_ERROR},
      Storage::FileSystem::{
        CreateFileW, GetFinalPathNameByHandleW, OpenFileById, FILE_FLAG_BACKUP_SEMANTICS,
        FILE_ID_128, FILE_ID_DESCRIPTOR, FILE_NAME_NORMALIZED, FILE_READ_ATTRIBUTES,
        FILE_SHARE_READ, OPEN_EXISTING, READ_CONTROL, SYNCHRONIZE,
      },
      System::WindowsProgramming::VOLUME_NAME_DOS,
    };

    let mut wstr: Vec<u16> = hint_dir.as_os_str().encode_wide().collect();
    // terminal character
    wstr.push(0x0000);

    // open hint directory for OpenFileById()
    let h_dir_hint: HANDLE = unsafe {
      CreateFileW(
        wstr.as_ptr(),
        READ_CONTROL,
        FILE_SHARE_READ,
        std::ptr::null(),
        OPEN_EXISTING,
        FILE_FLAG_BACKUP_SEMANTICS,
        0,
      )
    };

    if h_dir_hint == INVALID_HANDLE_VALUE {
      return Err(format!("drive open failed. code: {}", unsafe { GetLastError() }).to_string());
    }

    let mut id = FILE_ID_128 {
      Identifier: Default::default(),
    };

    // String -> [u8;  16]
    for (i, c1) in id_str.chars().rev().enumerate() {
      // odd check
      if i & 0x1 == 1 {
        let c2 = id_str.chars().rev().nth(i - 1).unwrap();
        // '0' -> 0
        id.Identifier[i >> 1] = ((c1 as u8 - '0' as u8) << 4) | (c2 as u8 - '0' as u8);
      }
    }

    let id_descriptor = FILE_ID_DESCRIPTOR {
      dwSize: mem::size_of::<FILE_ID_DESCRIPTOR>() as u32,
      Type: 2,
      Anonymous: windows_sys::Win32::Storage::FileSystem::FILE_ID_DESCRIPTOR_0 {
        ExtendedFileId: id,
      },
    };

    let h_file = unsafe {
      OpenFileById(
        h_dir_hint,
        &id_descriptor,
        SYNCHRONIZE | FILE_READ_ATTRIBUTES,
        FILE_SHARE_READ,
        std::ptr::null(),
        0,
      )
    };

    if h_file == INVALID_HANDLE_VALUE {
      return Err(
        format!("directory open failed. code: {}", unsafe { GetLastError() }).to_string(),
      );
    }

    unsafe { CloseHandle(h_dir_hint) };

    let mut name = vec![0u16; 512];
    let length = unsafe {
      GetFinalPathNameByHandleW(
        h_file,
        name.as_mut_ptr(),
        name.capacity() as u32,
        FILE_NAME_NORMALIZED | VOLUME_NAME_DOS,
      )
    };

    if unsafe { GetLastError() } != NO_ERROR {
      let n = unsafe { GetLastError() };
      return Err(format!("GetFinalPathNameByHandleW() failed. code: {}", n));
    }

    unsafe { CloseHandle(h_file) };

    // drop '\0' elements
    name.truncate(length as usize);
    let name_str = String::from_utf16(&name).unwrap();
    let path = Path::new(&name_str);

    Ok(
      path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string(),
    )
  }

  #[cfg(target_os = "linux")]
  pub fn get_id_by_filename(path: &Path) -> Result<String, String> {
    Ok("todo")
  }
}

#[cfg(test)]
mod tests {

  use windows_sys::Win32::Foundation::{
    CloseHandle, GetLastError, HANDLE, INVALID_HANDLE_VALUE, NO_ERROR,
  };
  use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_SHARE_READ, OPEN_EXISTING,
  };

  use crate::filesystem::FileSystem;
  use std::{fs::File, io::Write, os::windows::prelude::OsStrExt, path::Path};

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

  #[test]
  fn test_file_id() {
    let filename = "README.md";
    let id = FileSystem::get_id_by_filename(Path::new(filename));
    let actual_filename = FileSystem::get_filename_by_id(&id.unwrap(), Path::new("."));
    assert_eq!(filename, actual_filename.unwrap());
  }
}

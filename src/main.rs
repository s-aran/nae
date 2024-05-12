use std::env;
use std::path;

fn main() {
  println!("Hello, world!");

  for (i, a) in std::env::args().collect::<Vec<String>>().iter().enumerate() {
    println!("{}: {}", i, a);
  }

  /*
  let (args, _) = opts! {
      synopsis "file renamer program.";
      auto_shorts  false;
      // opt verbose:Option<String> = Some("".to_string()), desc:"be verbose.", short: 'v';
      opt v:bool, desc: "be verbose.";
      opt vv:bool, desc: "be verbose.";
      opt vvv:bool, desc: "be verbose.";
      opt n:bool=false, desc: "enable dry run mode.";
      opt depth:i32=0, desc: "max directory depth.";
      opt file_type:Option<String>, desc: "file type. set [d]irectory, [f]ile.", long: "type";
      param pattern:String, desc: "filtering pattern.";
      opt e:bool=false, desc: "enable regex mode for pattern parameter.";
      param to:String, desc: "new file name";
      opt version:bool=false, desc: "show version information.";
  }
  .parse_or_exit();
  */

  // let verbose = args.verbose.unwrap_or("".to_string());
  // if verbose.len() > 0 {
  //   println!("verbose: {}", verbose);
  // }

  let target = path::PathBuf::from("./");
  // enumFiles(&target);
}

fn enum_files(target: &path::PathBuf) {
  let files = target.read_dir().unwrap();

  for dir_entry in files {
    let dir_entry = dir_entry.unwrap();
    let path = dir_entry.path();

    if path.is_dir() {
      enum_files(&path);
    } else {
      println!("{}", path.to_str().unwrap());
    }
  }
}

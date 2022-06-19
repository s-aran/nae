use std::{env, process::exit};

use rustop::opts;

fn main() {
  let a = opts! {
    auto_shorts false;
    synopsis "file renamer";
    help true;

    opt version:bool, short: 'V', long:"version", desc: "show version";
    opt verbose:bool, short: 'v', long:"verbose", desc: "verbose mode";
    opt dry_run:bool, long: "dry-run", desc: "dry run";
    param path:String, name: "<path>", desc: "rename path";
    param name:String, name: "<name>", desc: "renamed name";
  };

  let env_args = env::args().collect::<Vec<String>>();
  let  ea: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
  let (args, _) = match a.parse_args(ea.into_iter()) {
    Ok(a) => a,
    Err(rustop::Error::Help(msg)) => {
      println!("{}", msg);
      exit(1);
    }
    Err(e) => rustop::error_and_exit(&e),
  };

  if args.version {
    println!("nae version {}", env!("CARGO_PKG_VERSION"));
    exit(0);
  }

  if args.verbose {
    println!("verbose.");
  }

  println!("Hello, world!");
  println!("{:?} -> {}", args.path, args.name);
}

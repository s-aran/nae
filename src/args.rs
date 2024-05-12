use std::{borrow::BorrowMut, env};

use clap::{Parser, ValueEnum};

struct Argument {
  value: &'static str,
  help: String,
  long: String,
  short: char,
  required: bool,
}

impl Argument {
  pub fn new(
    value: &'static str,
    help: &String,
    long: &String,
    short: char,
    required: bool,
  ) -> Self {
    Self {
      value,
      help: help.to_string(),
      long: long.to_string(),
      short,
      required,
    }
  }
}

struct Arguments {
  args: Vec<Argument>,
}

// impl Arguments {
//   pub fn new(cmd_args: &std::env::Args) -> Self {
//     let mut args = vec![];
//     for a in cmd_args.collect::<Vec<String>>() {}
//
//     Self { args: args }
//   }
// }

#[derive(Debug, Clone, PartialEq, ValueEnum)]
enum ItemType {
  File,
  Directory,
}

#[derive(Parser, Debug)]
#[command(name = "nae", author = "s.aran", version = "0.90")]
struct Args {
  src: String,

  #[arg(short = 't', default_value = "f", value_enum)]
  item_type: String,

  #[arg(short = 'r', default_value_t = 1)]
  recursive: u8,

  #[arg(short = 'v', default_value_t = 0)]
  verbose: u8,
}

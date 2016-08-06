extern crate time;

use std::io::stdin;
use std::env::args;
use time::{strftime, now};

fn main() {
  let mut args = args();
  let time_format = args.nth(1).unwrap_or(String::from("%c"));
  let time_format = time_format.as_str();
  let stdin = stdin();
  loop {
    let mut line = String::new();
    let result = match stdin.read_line(&mut line) {
      Ok(r) => r,
      Err(e) => {
        println!("error reading stdin: {}", e);
        return;
      }
    };
    if result == 0 {
      break;
    }
    let timestamp = match strftime(time_format, &now()) {
      Ok(t) => t,
      Err(e) => {
        println!("invalid timestamp format: {}", e);
        return;
      }
    };
    print!("[{}] {}", timestamp, line);
  }
}

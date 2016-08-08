extern crate time;

use std::env::args;
use std::io::{stdin, stderr, Write};
use time::{strftime, now};

fn get_time_format() -> String {
  let mut args = args();
  args.nth(1).unwrap_or_else(|| String::from("%c"))
}

fn append_timestamp_to_stdin_yo() -> Result<(), String> {
  let time_format = &get_time_format();
  let stdin = stdin();
  loop {
    let mut line = String::new();
    let result = match stdin.read_line(&mut line) {
      Ok(r) => r,
      Err(e) => return Err(format!("error reading stdin: {}", e))
    };
    if result == 0 {
      break;
    }
    let timestamp = match strftime(time_format, &now()) {
      Ok(t) => t,
      Err(e) => return Err(format!("invalid timestamp format: {}", e))
    };
    print!("[{}] {}", timestamp, line);
  }
  Ok(())
}

fn inner() -> i32 {
  match append_timestamp_to_stdin_yo() {
    Err(e) => {
      let stderr = stderr();
      stderr.lock().write(e.as_bytes()).expect("error writing to stderr");
      1
    },
    Ok(_) => 0
  }
}

fn main() {
  let exit_code = inner();
  std::process::exit(exit_code);
}

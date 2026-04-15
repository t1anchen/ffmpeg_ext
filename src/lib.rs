use std::{
  error::Error,
  io::{BufRead, BufReader},
  process::Command,
};

use tracing::{debug, info};

use crate::cli::CmdRun;

pub mod datetime;
pub mod cli;
pub mod mediafile;

pub fn engine_run(
  engine_program: &str,
  engine_args: Vec<String>,
) -> Result<(), Box<dyn Error>> {
  let mut engine = Command::new(engine_program)
    .args(engine_args)
    .stderr(std::process::Stdio::piped())
    .spawn()?;

  if let Some(stderr) = engine.stderr.take() {
    let reader = BufReader::new(stderr);
    for line in reader.lines() {
      match line {
        Ok(line) => println!("{}", line),
        Err(_) => break,
      }
    }
  }

  Ok(())
}

pub fn api_main(opts: cli::Opts) -> Result<(), Box<dyn Error>> {
  opts.action.run(&opts)
}

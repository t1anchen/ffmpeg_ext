use std::{
  error::Error,
  io::{BufRead, BufReader},
  process::Command,
};

use tracing::{debug, info};

pub mod chrono;
pub mod cli;
pub mod duration;
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
  debug!("{:?}", opts);

  if opts.dryrun {
    info!("{:?}", opts);
    info!(
      "{} {}",
      opts.program.clone(),
      opts.to_args().clone().join(" ")
    );
    return Ok(());
  }

  let ffmpeg_args = opts.to_args();
  debug!("ffmpeg_args: {:?}", ffmpeg_args.clone());

  Ok(())
}

use std::{cell::RefCell, rc::Rc, string};

use clap::Parser;
use hello::Task;
use tokio::{
  io::{AsyncBufReadExt, BufReader},
  process::Command,
};
use tracing::debug;

pub mod hello;

#[derive(Debug, Clone, Parser, Default)]
#[command(about)]
pub struct Opts {
  #[arg(long, default_value = "false")]
  gui: bool,

  #[arg(long)]
  input_path: Option<String>,

  #[arg(long)]
  output_path: Option<String>,

  #[arg(short, long, default_value = "false")]
  verbose: bool,
}

impl Opts {
  fn to_args(&self) -> Vec<String> {
    let mut args = Vec::new();

    self.input_path.as_ref().map(|p| {
      args.push("-i".to_owned());
      args.push(p.clone());
    });
    self.output_path.as_ref().map(|p| {
      args.push(p.clone());
    });
    return args;
  }
}

pub async fn api_main(mut opts: Opts) -> anyhow::Result<()> {
  let opts = Rc::new(RefCell::new(&mut opts));
  debug!("{:?}", opts);

  let ffmpeg_args = opts.borrow().to_args();

  let mut engine = Command::new("ffmpeg")
    .args(&ffmpeg_args)
    .stderr(std::process::Stdio::piped())
    .spawn()?;

  if let Some(stderr) = engine.stderr.take() {
    let reader = BufReader::new(stderr);
    let mut streamed_lines = reader.lines();
    while let Some(line) = streamed_lines.next_line().await? {
      println!("{}", line)
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::Opts;

  #[test]
  fn unit_test() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn to_args_test() {
    let opts = Opts {
      gui: false,
      verbose: false,
      input_path: Some("/path/from".to_owned()),
      output_path: Some("/path/to".to_owned()),
    };
    let actual = opts.to_args().join(" ");
    let expected = "-i /path/from /path/to";
    assert_eq!(actual, expected);
  }
}

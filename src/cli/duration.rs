use std::path::PathBuf;

use clap::Parser;
use tracing::info;

use crate::{
  cli::{ArgsBuilder, CmdRun},
  engine_run,
};

#[derive(Debug, Clone, Parser)]
pub struct DurationCmd {
  #[arg(long, default_value = "ffprobe")]
  program: String,

  #[arg(short, long)]
  input_path: Option<PathBuf>,
}

impl ArgsBuilder for DurationCmd {
  fn build_args(&self, args: &mut Vec<String>) {
    if self.program.as_str().to_lowercase() == "ffprobe" {
      args.extend(vec![
        "-v".into(),
        "error".into(),
        "-show_entries".into(),
        "format=duration".into(),
        "-of".into(),
        "default=noprint_wrappers=1:nokey=1".into(),
      ]);
    }
  }
  fn to_args(&self) -> Vec<String> {
    let mut args: Vec<String> = vec![];
    args.push("-hide_banner".into());
    self.build_args(&mut args);
    if let Some(ref p) = self.input_path {
      args.extend(vec!["-i".into(), p.display().to_string()]);
    }
    args
  }
}

impl CmdRun for DurationCmd {
  fn run(&self, opts: &super::Opts) -> Result<(), Box<dyn std::error::Error>> {
    if opts.dryrun {
      info!(
        "{:#?}",
        [vec![self.program.to_string()], self.to_args()].concat()
      );
      Ok(())
    } else {
      engine_run(&self.program, self.to_args())
    }
  }
}

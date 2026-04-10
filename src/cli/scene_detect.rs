use std::path::PathBuf;

use clap::Parser;

use crate::{
  cli::{ArgsBuilder, CmdRun},
  engine_run,
};

#[derive(Clone, Parser, Debug)]
pub struct SceneDetectCmd {
  #[arg(long, default_value = "scenedetect")]
  program: String,

  #[arg(long)]
  input_path: Option<PathBuf>,

  #[arg(long)]
  output_path: Option<PathBuf>,
  #[arg(long)]
  detect_content: bool,
  #[arg(long)]
  list_scenes: bool,
  #[arg(long)]
  split_video: bool,
}

impl ArgsBuilder for SceneDetectCmd {
  fn build_args(&self, args: &mut Vec<String>) {
    if self.detect_content {
      args.push("detect-content".into());
    }
    if self.list_scenes {
      args.push("list-scene".into());
    }
    if self.split_video {
      args.push("split-video".into());
    }
  }
  fn to_args(&self) -> Vec<String> {
    let mut args: Vec<String> = vec![];
    if let Some(ref p) = self.input_path {
      args.extend(vec!["--input".into(), p.display().to_string()]);
    }
    self.build_args(&mut args);
    args
  }
}

impl CmdRun for SceneDetectCmd {
  fn run(&self, opts: &super::Opts) -> Result<(), Box<dyn std::error::Error>> {
    engine_run(&self.program, self.to_args())
  }
}

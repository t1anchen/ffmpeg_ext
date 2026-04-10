use std::{error::Error, path::PathBuf};

use clap::Parser;

use crate::{
  cli::{ArgsBuilder, CmdRun},
  engine_run,
};

#[derive(Clone, Parser, Debug)]
pub struct SplitByTimeCmd {
  #[arg(long, default_value = "ffmpeg")]
  program: String,

  #[arg(long)]
  input_path: Option<PathBuf>,

  #[arg(long)]
  output_path: Option<PathBuf>,

  #[arg(long)]
  start_time: String,
  #[arg(long, default_value = "2")]
  video_quality: u8,
  #[arg(long, default_value = "image2")]
  output_format: String,
  #[arg(long, default_value = "1")]
  video_frame: u8,
  #[arg(long, default_value = ".png")]
  output_suffix: String,
  #[arg(long, default_value = "-1")]
  width_scale: i8,
  #[arg(long, default_value = "-1")]
  height_scale: i8,
}

impl ArgsBuilder for SplitByTimeCmd {
  fn build_args(&self, args: &mut Vec<String>) {
    args.extend(vec![
      "-nostdin".into(),
      "-stats".into(),
      "-v".into(),
      "panic".into(),
      "-ss".into(),
      self.start_time.clone(),
      "-q:v".into(),
      self.video_quality.to_string(),
      "-f".into(),
      self.output_format.clone(),
      "-vframes".into(),
      self.video_frame.to_string(),
      "-vf".into(),
      format!("scale={}:{}", self.width_scale, self.height_scale),
    ]);
  }

  fn to_args(&self) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    args.push("-hide_banner".into());
    if let Some(ref p) = self.input_path {
      args.extend(vec!["-i".into(), p.display().to_string()]);
    }
    self.build_args(&mut args);
    if let Some(ref p) = self.output_path {
      args.push(p.display().to_string());
    }
    args
  }
}

impl CmdRun for SplitByTimeCmd {
  fn run(&self, opts: &super::Opts) -> Result<(), Box<dyn Error>> {
    engine_run(&self.program, self.to_args())
  }
}

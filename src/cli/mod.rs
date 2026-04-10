use std::{error::Error, path::PathBuf};

pub mod merge_segments;
pub mod scene_detect;
pub mod split_by_time;

use clap::{Parser, Subcommand};
use merge_segments::MergeSegmentsCmd;
use scene_detect::SceneDetectCmd;
use split_by_time::SplitByTimeCmd;

#[derive(Debug, Clone, Parser, Default)]
#[command(version, about, long_about = None)]
pub struct Opts {
  #[arg(long, default_value = "ffmpeg")]
  pub program: String,

  #[arg(long, default_value = "false")]
  pub gui: bool,

  #[arg(long)]
  pub input_path: Option<PathBuf>,

  #[arg(long)]
  pub output_path: Option<PathBuf>,

  #[command(subcommand)]
  pub action: Option<Feature>, // 现在引用自 commands 模块

  #[arg(short, long, global = true, default_value = "false")]
  pub verbose: bool,

  #[arg(long, global = true, default_value = "false")]
  pub dryrun: bool,
}

impl Opts {
  pub fn to_args(&self) -> Vec<String> {
    let mut args = Vec::new();
    match self.program.as_str() {
      "ffmpeg" => {
        args.push("-hide_banner".into());
        if let Some(ref p) = self.input_path {
          args.extend(vec!["-i".into(), p.display().to_string()]);
        }
        if let Some(ref action) = self.action {
          action.build_args(&mut args);
        }
        if let Some(ref p) = self.output_path {
          args.push(p.display().to_string());
        }
      }
      "scenedetect" => {
        if let Some(ref p) = self.input_path {
          args.extend(vec!["--input".into(), p.display().to_string()]);
        }
        if let Some(ref action) = self.action {
          action.build_args(&mut args);
        }
      }
      _ => {}
    }
    args
  }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Feature {
  SceneDetect(SceneDetectCmd),
  SplitByTime(SplitByTimeCmd),
  MergeSegments(MergeSegmentsCmd),
}

/// 定义参数构建行为的 Trait
pub trait ArgsBuilder {
  fn build_args(&self, args: &mut Vec<String>) {}
  fn to_args(&self) -> Vec<String> {
    vec![]
  }
}

pub trait CmdRun {
  fn run(&self, opts: &Opts) -> Result<(), Box<dyn Error>>;
}

impl ArgsBuilder for Feature {
  fn build_args(&self, args: &mut Vec<String>) {
    match self {
      Feature::SceneDetect(cmd) => cmd.build_args(args),
      Feature::SplitByTime(cmd) => cmd.build_args(args),
      Feature::MergeSegments(cmd) => cmd.build_args(args),
    }
  }
}

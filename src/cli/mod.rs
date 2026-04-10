use std::{error::Error, path::PathBuf};

pub mod duration;
pub mod merge_segments;
pub mod scene_detect;
pub mod split_by_time;

use clap::{Parser, Subcommand};
use duration::DurationCmd;
use merge_segments::MergeSegmentsCmd;
use scene_detect::SceneDetectCmd;
use split_by_time::SplitByTimeCmd;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Opts {
  #[arg(long, default_value = "false")]
  pub gui: bool,

  #[command(subcommand)]
  pub action: Feature, // 现在引用自 commands 模块

  #[arg(short, long, global = true, default_value = "false")]
  pub verbose: bool,

  #[arg(long, global = true, default_value = "false")]
  pub dryrun: bool,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Feature {
  SceneDetect(SceneDetectCmd),
  SplitByTime(SplitByTimeCmd),
  MergeSegments(MergeSegmentsCmd),
  Duration(DurationCmd),
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

impl CmdRun for Feature {
  fn run(&self, opts: &Opts) -> Result<(), Box<dyn Error>> {
    match self {
      Feature::SceneDetect(cmd) => cmd.run(opts),
      Feature::SplitByTime(cmd) => cmd.run(opts),
      Feature::MergeSegments(cmd) => cmd.run(opts),
      Feature::Duration(cmd) => cmd.run(opts),
    }
  }
}

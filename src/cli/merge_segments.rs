use std::{error::Error, path::Path};

use clap::Parser;
use tracing::info;

use crate::{
  cli::{ArgsBuilder, CmdRun, Opts},
  mediafile::{MediaFile, MediaFileAttribute, MediaFiles},
};

#[derive(Clone, Parser, Debug)]
pub struct MergeSegmentsCmd {
  #[arg(long, default_value = "ffmpeg")]
  program: String,

  #[arg(long, default_value = "copy")]
  mode: String,

  /// 媒体文件目录
  #[arg(short, long, default_value = "/home/xxx/media/")]
  input_dir: String,

  /// 输出目录
  #[arg(short, long, default_value = "./output/")]
  output_dir: String,

  /// 跳过已完成的合并任务
  #[arg(short, long, default_value = "true")]
  skip_completed: bool,

  /// 日志文件路径
  #[arg(long, default_value = "./merge_logs.txt")]
  log_file: String,
}

impl ArgsBuilder for MergeSegmentsCmd {}

impl CmdRun for MergeSegmentsCmd {
  fn run(&self, opts: &Opts) -> Result<(), Box<dyn Error>> {
    // scan and grouping
    let maybe_media_files =
      MediaFile::from_scanning(Path::new(&self.input_dir));
    match maybe_media_files {
      Ok(media_files) => {
        let groups = MediaFiles::from_vec(media_files)
          .sort(MediaFileAttribute::Name)
          .to_groups();
        info!("{:#?}", groups);
        // merging
        for (i, group) in groups.into_iter().enumerate() {
          let group_idx = i + 1;
        }
      }
      Err(_) => (),
    };
    return Ok(());
  }
}

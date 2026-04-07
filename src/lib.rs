use std::path::{Path, PathBuf};

use anyhow::Ok;
use clap::Parser;
use tokio::{
  io::{AsyncBufReadExt, BufReader},
  process::Command,
};
use tracing::{debug, info};

pub mod chrono;
pub mod commands;
pub mod duration;
pub mod mediafile;

use crate::{
  commands::{ArgsBuilder, Commands},
  mediafile::MediaFile,
};

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
  pub action: Option<Commands>, // 现在引用自 commands 模块

  #[arg(short, long, default_value = "false")]
  pub verbose: bool,

  #[arg(long, default_value = "false")]
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

pub async fn api_main(mut opts: Opts) -> anyhow::Result<()> {
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

  if let Some(Commands::Merge {
    mode,
    input_dir,
    output_dir,
    skip_completed,
    log_file,
  }) = opts.action
  {
    info!("start");
    let maybe_media_files = MediaFile::from_scanning(Path::new(&input_dir));
    match maybe_media_files {
      std::result::Result::Ok(media_files) => info!("{:?}", media_files),
      Err(_) => (),
    };
    return Ok(());
  }

  let ffmpeg_args = opts.to_args();
  debug!("ffmpeg_args: {:?}", ffmpeg_args.clone());

  let mut engine = Command::new(opts.program.clone())
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
  use std::path::PathBuf;

  use crate::Opts;

  #[test]
  fn unit_test() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn to_args_test() {
    let opts = Opts {
      program: "ffmpeg".to_owned(),
      gui: false,
      verbose: false,
      input_path: Some("/path/from".into()),
      output_path: Some("/path/to".into()),
      action: None,
      dryrun: false,
    };
    let actual = opts.to_args().join(" ");
    let expected = "-hide_banner -i /path/from /path/to";
    assert_eq!(actual, expected);
  }
}

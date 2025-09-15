use std::{cell::RefCell, rc::Rc, string};

use anyhow::Ok;
use clap::{Parser, Subcommand};
use hello::Task;
use tokio::{
  io::{AsyncBufReadExt, BufReader},
  process::Command,
};
use tracing::{debug, info};

pub mod hello;

#[derive(Debug, Clone, Parser, Default)]
#[command(version, about, long_about = None)]
pub struct Opts {
  #[arg(long, default_value = "ffmpeg")]
  program: String,

  #[arg(long, default_value = "false")]
  gui: bool,

  #[arg(long)]
  input_path: Option<String>,

  #[arg(long)]
  output_path: Option<String>,

  #[command(subcommand)]
  action: Option<Commands>,

  #[arg(short, long, default_value = "false")]
  verbose: bool,

  #[arg(long, default_value = "false")]
  dryrun: bool,
}

#[derive(Debug, Clone, Subcommand)]
enum Commands {
  SceneDetect {
    #[arg(long)]
    detect_content: bool,

    #[arg(long)]
    list_scenes: bool,

    #[arg(long)]
    split_video: bool,
  },
}

impl Opts {
  fn calibrate(&mut self) {
    match self.action {
      Some(Commands::SceneDetect {
        detect_content,
        list_scenes,
        split_video,
      }) => {
        self.program = "scenedetect".to_owned();
      }
      None => {}
    }
  }
  fn to_args(&self) -> Vec<String> {
    let mut args = Vec::new();

    match &self.program[..] {
      "ffmpeg" => {
        self.input_path.as_ref().map(|p| {
          args.push("-i".to_owned());
          args.push(p.clone());
        });
        self.output_path.as_ref().map(|p| {
          args.push(p.clone());
        });
      }
      "scenedetect" => {
        self.input_path.as_ref().map(|p| {
          args.push("--input".to_owned());
          args.push(p.clone());
        });

        match self.action {
          Some(Commands::SceneDetect {
            detect_content,
            list_scenes,
            split_video,
          }) => {
            if detect_content {
              args.push("detect-content".to_owned());
            }
            if list_scenes {
              args.push("list-scene".to_owned());
            }
            if split_video {
              args.push("split-video".to_owned());
            }
          }
          None => {}
        }
      }
      _ => {}
    }

    return args;
  }
}

pub async fn api_main(mut opts: Opts) -> anyhow::Result<()> {
  let opts = Rc::new(RefCell::new(&mut opts));
  opts.borrow_mut().calibrate();
  debug!("{:?}", opts);

  if opts.borrow().dryrun {
    info!("{:?}", opts);
    return Ok(());
  }

  let ffmpeg_args = opts.borrow().to_args();

  let mut engine = Command::new(opts.borrow().program.clone())
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
      program: "ffmpeg".to_owned(),
      gui: false,
      verbose: false,
      input_path: Some("/path/from".to_owned()),
      output_path: Some("/path/to".to_owned()),
      action: None,
      dryrun: false,
    };
    let actual = opts.to_args().join(" ");
    let expected = "-i /path/from /path/to";
    assert_eq!(actual, expected);
  }
}

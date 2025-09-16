use std::{cell::RefCell, rc::Rc};

use anyhow::Ok;
use clap::{Parser, Subcommand};
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
  SplitByTime {
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
  },
}

impl Opts {
  fn calibrate(&mut self) {
    match self.action {
      Some(Commands::SceneDetect {
        detect_content: _,
        list_scenes: _,
        split_video: _,
      }) => {
        self.program = "scenedetect".to_owned();
      }
      _ => {}
    }
  }
  fn to_args(&self) -> Vec<String> {
    let mut args = Vec::new();

    match &self.program[..] {
      "ffmpeg" => {
        // hide banner
        args.push("-hide_banner".to_owned());
        // input
        self.input_path.as_ref().map(|p| {
          args.push("-i".to_owned());
          args.push(p.clone());
        });
        let mut output_path = self.output_path.clone();
        // subcommands
        match &self.action {
          // split-by-time
          Some(Commands::SplitByTime {
            start_time,
            video_quality,
            output_format,
            video_frame,
            output_suffix,
            width_scale,
            height_scale,
          }) => {
            // misc and logging
            args.push("-nostdin".to_owned());
            args.push("-stats".to_owned());
            args.push("-v".to_owned());
            args.push("panic".to_owned());
            // start time
            args.push("-ss".to_owned());
            args.push(start_time.clone());
            // output video quality
            args.push("-q:v".to_owned());
            args.push(video_quality.to_string());
            // output format
            args.push("-f".to_owned());
            args.push(output_format.clone());
            // video frame
            args.push("-vframes".to_owned());
            args.push(video_frame.to_string());
            // scale
            args.push("-vf".to_owned());
            //   [2025-09-16T22:06:54+08:00] NEVER add double quotes in this
            //   term, I was hallucinated by the hints from Perplexity.AI, and
            //   it wasted my 2 hours to fix it.
            args.push(format!("scale={}:{}", width_scale, height_scale));
            // default output path
            if output_path.is_none() {
              let mut output_name = vec![];
              self.input_path.as_ref().map(|p| {
                let input_path = std::path::Path::new(&p);
                input_path
                  .file_stem()
                  .map(|s| s.to_str())
                  .flatten()
                  .map(|stem_str| stem_str.to_string())
                  .map(|stem| {
                    output_name.push(stem);
                  });
              });

              // output_name.push(start_time.to_string());
              output_name.push(output_format.to_string());

              output_path = Some(output_name.join("-") + output_suffix)
            }
          }
          _ => {}
        }
        // output
        output_path.as_ref().map(|p| {
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
          _ => {}
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
    info!(
      "{} {}",
      opts.borrow().program.clone(),
      opts.borrow().to_args().clone().join(" ")
    );
    return Ok(());
  }

  let ffmpeg_args = opts.borrow().to_args();
  debug!("ffmpeg_args: {:?}", ffmpeg_args.clone());

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
    let expected = "-hide_banner -i /path/from /path/to";
    assert_eq!(actual, expected);
  }
}

use clap::Subcommand;
use tracing::debug;

/// 定义参数构建行为的 Trait
pub trait ArgsBuilder {
  fn build_args(&self, args: &mut Vec<String>);
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
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
  Merge {
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
  },
}

impl ArgsBuilder for Commands {
  fn build_args(&self, args: &mut Vec<String>) {
    match self {
      Commands::SplitByTime {
        start_time,
        video_quality,
        output_format,
        video_frame,
        width_scale,
        height_scale,
        ..
      } => {
        args.extend(vec![
          "-nostdin".into(),
          "-stats".into(),
          "-v".into(),
          "panic".into(),
          "-ss".into(),
          start_time.clone(),
          "-q:v".into(),
          video_quality.to_string(),
          "-f".into(),
          output_format.clone(),
          "-vframes".into(),
          video_frame.to_string(),
          "-vf".into(),
          format!("scale={}:{}", width_scale, height_scale),
        ]);
      }
      Commands::SceneDetect {
        detect_content,
        list_scenes,
        split_video,
      } => {
        if *detect_content {
          args.push("detect-content".into());
        }
        if *list_scenes {
          args.push("list-scene".into());
        }
        if *split_video {
          args.push("split-video".into());
        }
      }
      Commands::Merge {
        mode,
        input_dir,
        output_dir,
        skip_completed,
        log_file,
      } => {
        debug!("{:?}", args)
      }
    }
  }
}

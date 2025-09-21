use std::path::Path;

use clap::builder::Str;
use tokio_stream::StreamExt;

use crate::engine::async_execute;

pub enum MediaType {
  // Audio
  AAC,
  MP3,
  M4A,
  OGA,
  OPUS,
  // Video
  AVI,
  MP4,
  OGG,
}

pub enum EncodingType {
  // Audio
  MP3,
  VORBIS,
  OPUS,
  // Video
  H264,
  H265,
  AV1,
  VP9,
}

pub struct MediaFile {}

impl MediaFile {
  // detect if file is media file via ffprobe
  pub async fn is_media_file(file_path: &Path) -> bool {
    let mut result = true;
    let file_path_str = file_path.to_str().map(String::from).unwrap();
    let stream = async_execute(
      "ffprobe",
      vec![
        format!("-loglevel"),
        format!("error"),
        format!("-show_entries"),
        format!("stream=codec_type"),
        format!("-of"),
        format!("default=nw=1"),
        file_path_str,
      ],
    )
    .await;
    tokio::pin!(stream);
    while let Some(Ok(line)) = stream.next().await {
      if line.contains("Invalid data found") {
        result = false;
        break;
      }
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  #[ignore = "test media file required"]
  async fn is_media_file_testing() {
    let actual =
      MediaFile::is_media_file(&Path::new("Sintel.2010.720p.mkv")).await;
    assert!(actual);
    let actual = MediaFile::is_media_file(&Path::new("README.md")).await;
    assert!(!actual)
  }
}

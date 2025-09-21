use std::path::Path;

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
  pub fn is_media_file(file_path: &Path) -> bool {
    // detect if media file
    //
    // 1. from extension
    // 2. from metadata
    false
  }
}

use std::{
  error::Error,
  fs,
  path::{Path, PathBuf},
  process::Command,
  time::SystemTime,
};

use tracing::{debug, info};

#[derive(Debug, Clone)]
pub struct MediaFile {
  path: PathBuf,
  duration_in_secs: u32,
  size_in_mb: f64,
}

impl MediaFile {
  fn is_full_segment(&self, threshold: impl Into<Option<u32>>) -> bool {
    let limit = threshold.into().unwrap_or(60);
    self.duration_in_secs == limit
  }

  fn get_duration_in_secs(path: &Path) -> Result<u32, Box<dyn Error>> {
    let mut n_retries = 3;
    while n_retries > 0 {
      let output_from_ffprobe = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-show_entries")
        .arg("format=duration")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(path)
        .output();

      match output_from_ffprobe {
        Ok(out) => {
          if out.status.success() {
            let dur_str =
              String::from_utf8_lossy(&out.stdout).trim().to_string();
            let sec: f64 = dur_str.parse().unwrap_or(0.0);
            return Ok(sec.ceil() as u32);
          } else {
            n_retries -= 1;
            std::thread::sleep(std::time::Duration::from_secs(1));
          }
        }
        Err(_) => {
          n_retries -= 1;
          std::thread::sleep(std::time::Duration::from_secs(1));
        }
      }
    }
    Err(format!("Reach maximum of retries: {}", path.display()).into())
  }

  // ============================== STATIC METHODS =============================

  fn get_size_mb(path: &Path) -> Result<f64, Box<dyn Error>> {
    let metadata = fs::metadata(path)?;
    let size_bytes = metadata.len() as f64;
    Ok(size_bytes / 1024.0 / 1024.0)
  }

  pub fn from_scanning(
    input_path: &Path,
  ) -> Result<Vec<MediaFile>, Box<dyn Error>> {
    let mut media_files = Vec::new();
    let dir_path = Path::new(input_path);

    for entry in fs::read_dir(dir_path)? {
      let entry = entry?;
      let path = entry.path();

      // 只处理 MP4 文件
      if path.is_file()
        && path
          .extension()
          .unwrap_or_default()
          .to_str()
          .unwrap_or("")
          .to_lowercase()
          == "mp4"
      {
        let size_in_mb = MediaFile::get_size_mb(&path)?;
        let duration_in_secs = MediaFile::get_duration_in_secs(&path)?;

        media_files.push(MediaFile {
          path,
          duration_in_secs,
          size_in_mb,
        });
      }
    }

    Ok(media_files)
  }
}

pub enum MediaFileAttribute {
  Name,
  LastCreationTime,
  LastModifiedTime,
}

pub struct MediaFiles {
  files: Vec<MediaFile>,
}
impl MediaFiles {
  pub fn from_vec(files: Vec<MediaFile>) -> MediaFiles {
    MediaFiles { files }
  }

  pub fn sort(&mut self, by: MediaFileAttribute) -> &mut Self {
    match by {
      MediaFileAttribute::Name => self.sort_by_name(),
      MediaFileAttribute::LastCreationTime => self.sort_by_creation_time(),
      MediaFileAttribute::LastModifiedTime => self.sort_by_modified_time(),
    }
    self
  }

  fn sort_by_name(&mut self) {
    self
      .files
      .sort_by_key(|mf| mf.path.to_string_lossy().to_string());
  }

  fn sort_by_creation_time(&mut self) {
    self.files.sort_by_key(|mf| {
      fs::metadata(&mf.path)
        .and_then(|m| m.created())
        .unwrap_or(SystemTime::UNIX_EPOCH)
    });
  }

  fn sort_by_modified_time(&mut self) {
    self.files.sort_by_key(|mf| {
      fs::metadata(&mf.path)
        .and_then(|m| m.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH)
    });
  }

  pub fn to_groups(&self) -> Vec<Vec<MediaFile>> {
    let mut groups = Vec::new();
    let mut current_group = Vec::new();

    for file in &self.files {
      current_group.push(file.clone());

      // 遇到不足1分钟的文件，闭合分组
      if !file.is_full_segment(None) {
        groups.push(current_group);
        current_group = Vec::new();
      }
    }

    // 处理最后一组（如果最后一个文件是1分钟，也单独作为一组）
    if !current_group.is_empty() {
      groups.push(current_group);
    }

    groups
  }
}

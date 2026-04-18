use std::{
  error::Error,
  fs::{self, File, FileTimes},
  io::{BufWriter, Write},
  os::windows::fs::FileTimesExt,
  path::{Path, PathBuf},
  process::{Command, Stdio},
  time::SystemTime,
};

use tracing::{debug, error, info};
use tracing_subscriber::fmt::format;

use crate::datetime::SimpleDateTime;

#[derive(Debug, Clone)]
pub struct MediaFile {
  path: PathBuf,
  duration_in_secs: u32,
  size_in_mb: f64,
  name: String,
  last_created: Option<SystemTime>,
  last_modified: Option<SystemTime>,
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
        let name = path
          .file_stem()
          .and_then(|os_str| os_str.to_str())
          .map(|s| s.to_string())
          .unwrap();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified().ok();
        let last_created = metadata.created().ok();

        media_files.push(MediaFile {
          path,
          duration_in_secs,
          size_in_mb,
          name,
          last_created,
          last_modified,
        });
      }
    }

    Ok(media_files)
  }
}

#[derive(Debug, Clone)]
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

  pub fn to_groups(&self) -> Vec<MediaFileGroup> {
    let mut groups: Vec<MediaFileGroup> = Vec::new();
    let mut current_group: Vec<MediaFile> = Vec::new();

    for file in &self.files {
      current_group.push(file.clone());

      // 遇到不足1分钟的文件，闭合分组
      if !file.is_full_segment(None) {
        groups.push(MediaFileGroup::from_vec(current_group));
        current_group = Vec::new();
      }
    }

    // 处理最后一组（如果最后一个文件是1分钟，也单独作为一组）
    if !current_group.is_empty() {
      groups.push(MediaFileGroup::from_vec(current_group));
    }

    groups
  }
}

#[derive(Debug)]
pub struct MediaFileGroup {
  pub files: Vec<MediaFile>,
  pub name: String,
}

impl MediaFileGroup {
  pub fn from_vec(files: Vec<MediaFile>) -> MediaFileGroup {
    let mut group = MediaFileGroup {
      files,
      name: "Untitled".into(),
    };
    // TODO: sort by conditions: timestamp or name
    group.name = group.new_name();
    group
  }
  fn the_earlist(&self) -> Option<&MediaFile> {
    self.files.first()
  }
  fn the_latest(&self) -> Option<&MediaFile> {
    self.files.last()
  }
  pub fn diff3_try_from_date_strs<'a>(
    s1: &'a str,
    s2: &'a str,
  ) -> (&'a str, &'a str, &'a str, &'a str) {
    let (common_prefix, s1_uniq, s2_uniq) = Self::diff3_from_strs(s1, s2);
    let is_valid_datetime = |s: &str| {
      SimpleDateTime::from_str("%Y%m%d%H%M%S", &s[..14], 8.0).is_some()
    };
    if s1.len() >= 14 && s2.len() >= 14 {
      if is_valid_datetime(s1) && is_valid_datetime(s2) {
        return match common_prefix.len() {
          0..8 => ("", &s1[..14], &s2[..14], "p0800"), // not same day
          8..14 => (&s1[..8], &s1[8..], &s2[8..], "p0800"), // same day
          _ => (common_prefix, s1_uniq, s2_uniq, "p0800"),
        };
      }
    }
    (common_prefix, s1_uniq, s2_uniq, "")
  }
  pub fn diff3_from_strs<'a>(
    s1: &'a str,
    s2: &'a str,
  ) -> (&'a str, &'a str, &'a str) {
    // 1. Find the first byte index where they differ
    let divergence_idx = s1
      .char_indices()
      .zip(s2.chars())
      .take_while(|((_, c1), c2)| c1 == c2)
      .map(|((idx, c), _)| idx + c.len_utf8())
      .last()
      .unwrap_or(0);

    // 2. Slice the original strings based on that index
    let prefix = &s1[..divergence_idx];
    let rem1 = &s1[divergence_idx..];
    let rem2 = &s2[divergence_idx..];

    (prefix, rem1, rem2)
  }
  pub fn new_name(&self) -> String {
    if let (Some(earlist), Some(latest)) =
      (self.the_earlist(), self.the_latest())
    {
      let s1 = &earlist.name;
      let s2 = &latest.name;
      let (common_prefix, earlist_part, latest_part, common_suffix) =
        MediaFileGroup::diff3_try_from_date_strs(s1, s2);
      return match common_prefix.len() {
        0 => format!("{}--{}-{}", earlist_part, latest_part, common_suffix),
        _ => format!(
          "{}-{}--{}-{}",
          common_prefix, earlist_part, latest_part, common_suffix
        ),
      };
    }
    "Untitle".into()
  }

  pub fn to_filelist(&self) -> Result<PathBuf, Box<dyn Error>> {
    let filelist_filename = format!("{}.txt", self.name);
    let filelist_path = Path::new(&filelist_filename);
    let filelist_file = std::fs::File::create(filelist_path)?;
    let mut writer = BufWriter::new(filelist_file);

    for media_file in self.files.iter() {
      let row = format!(
        "file '{}'",
        media_file.path.display().to_string().replace('\\', "/")
      );
      writeln!(writer, "{}", row)?;
    }
    Ok(filelist_path.to_path_buf())
  }

  pub fn merge(&self, output_path: &Path) -> Result<(), Box<dyn Error>> {
    let input_filelist = self.to_filelist()?;

    // construct command
    let mut cmd = Command::new("ffmpeg");
    cmd
      .args([
        "-f",
        "concat",
        "-safe",
        "0",
        "-i",
        input_filelist.to_str().unwrap(),
        "-c",
        "copy",
        "-y",
        output_path.to_str().unwrap(),
      ])
      .stdout(Stdio::piped())
      .stderr(Stdio::piped());

    // send to ffmpeg to execute
    let proc = cmd.spawn()?;
    let output = proc.wait_with_output()?;

    // execute result
    let _ = fs::remove_file(input_filelist.as_path());
    if !output.status.success() {
      let err = String::from_utf8_lossy(&output.stderr);
      error!("ffmpeg failed: {}", err);
      return Err(format!("ffmpeg failed and return {}", output.status).into());
    }

    // last modified
    let now = SystemTime::now();
    let output_file = fs::OpenOptions::new().write(true).open(output_path)?;
    let mut timestamp = FileTimes::new();
    if let Some(last_modified_from_latest) =
      self.the_latest().and_then(|latest| {
        fs::metadata(latest.path.as_path())
          .and_then(|m| m.modified())
          .ok()
      })
    {
      timestamp = timestamp.set_modified(last_modified_from_latest);
    } else {
      timestamp = timestamp.set_modified(now);
    }

    #[cfg(windows)]
    {
      if let Some(last_created_from_earlist) =
        self.the_earlist().and_then(|earlist| {
          fs::metadata(earlist.path.as_path())
            .and_then(|m| m.created())
            .ok()
        })
      {
        timestamp = timestamp.set_created(last_created_from_earlist);
      } else {
        timestamp = timestamp.set_created(now);
      }
    }

    output_file.set_times(timestamp)?;

    Ok(())
  }
}

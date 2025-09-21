use std::path::Path;

use crate::{Opts, media::MediaFile};

pub struct Duration {
  secs: usize,
}

impl Duration {
  pub async fn from(opts: &Opts) -> Self {
    let total_secs = 0;
    if let Some(input_path_string) = opts.input_path.clone() {
      let input_path = Path::new(&input_path_string);
      if let Ok(metadata) = input_path.metadata() {
        match metadata.file_type() {
          ft if ft.is_dir() => {
            for entry in input_path.read_dir().expect("readdir error") {
              if let Ok(entry) = entry {
                if MediaFile::is_media_file(&entry.path()).await {
                  // read media files from ffprobe and get duration
                  //
                  // ffprobe -i 'xxx.opus' -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1
                }
              }
            }
          }
          ft if ft.is_file() => {
            if MediaFile::is_media_file(input_path).await {
              // read media files form ffprobe and get duration
            }
          }
          _ => {}
        }
      }
    };
    Duration { secs: total_secs }
  }
}

#[cfg(test)]
mod tests {
  use crate::{Opts, duration::Duration};

  fn gen_stub_opts() -> Opts {
    Opts {
      program: "ffprobe".to_owned(),
      gui: false,
      input_path: None,
      output_path: None,
      action: None,
      verbose: false,
      dryrun: false,
    }
  }

  #[tokio::test]
  async fn ctor_test() {
    let d = Duration::from(&gen_stub_opts()).await;
    assert_eq!(d.secs, 0);
  }
}

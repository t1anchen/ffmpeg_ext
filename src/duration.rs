use std::path::Path;

use crate::Opts;

pub struct Duration {
  opts: Opts,
  secs: usize,
}

impl From<Opts> for Duration {
  fn from(opts: Opts) -> Self {
    Duration {
      opts: opts,
      secs: 0,
    }
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

  #[test]
  fn ctor_test() {
    let d = Duration::from(gen_stub_opts());
    assert_eq!(d.secs, 0);
  }
}

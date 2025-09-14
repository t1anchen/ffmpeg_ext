use std::{cell::RefCell, rc::Rc};

use clap::Parser;
use hello::Task;
use tracing::debug;

pub mod hello;

pub fn api_main(mut opts: Opts) {
  let opts = Rc::new(RefCell::new(&mut opts));
  debug!("{:?}", opts);
  
}

#[derive(Debug, Clone, Parser, Default)]
#[command(about)]
pub struct Opts {
  #[arg(long)]
  gui: bool,

  #[arg(short, long)]
  verbose: bool,
}

#[cfg(test)]
mod tests {
  #[test]
  fn unit_test() {
    assert_eq!(2 + 2, 4);
  }
}

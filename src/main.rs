use clap::Parser;
use hello_world::{Opts, api_main};
use tracing::info;
use tracing_subscriber;

fn main() {
  tracing_subscriber::fmt::init();

  let opts = Opts::parse();

  let start = std::time::Instant::now();
  api_main(opts);
  let elapsed = start.elapsed();
  info!("elapsed: {:?}", elapsed)
}

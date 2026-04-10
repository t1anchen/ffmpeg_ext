use std::error::Error;

use ffmpeg_ext::cli;

use clap::Parser;
use ffmpeg_ext::api_main;
use tracing::info;
use tracing_subscriber;

fn main() -> Result<(), Box<dyn Error>> {
  tracing_subscriber::fmt::init();

  let opts: cli::Opts = cli::Opts::parse();

  let start = std::time::Instant::now();
  let _ = api_main(opts);
  let elapsed = start.elapsed();
  info!("elapsed: {:?}", elapsed);

  Ok(())
}

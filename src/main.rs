use clap::Parser;
use ffmpeg_ext::{Opts, api_main};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
  tracing_subscriber::fmt::init();

  let opts = Opts::parse();

  let start = std::time::Instant::now();
  let _ = api_main(opts).await;
  let elapsed = start.elapsed();
  info!("elapsed: {:?}", elapsed);

  Ok(())
}

use async_stream::stream;
use futures_core::stream::Stream;
use tokio::{
  io::{self, AsyncBufReadExt, BufReader},
  process::Command,
};

pub async fn execute(
  program: &str,
  args: Vec<String>,
) -> impl Stream<Item = io::Result<String>> {
  stream! {
  let mut engine = Command::new(program)
    .args(&args)
    .stderr(std::process::Stdio::piped())
    .spawn()?;

  if let Some(stderr) = engine.stderr.take() {
    let reader = BufReader::new(stderr);
    let mut streamed_lines = reader.lines();
    while let Some(line) = streamed_lines.next_line().await? {
      yield Ok(line);
    }

  }

  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tokio_stream::StreamExt;

  #[tokio::test]
  async fn execute_testing() {
    let stream = execute("ffmpeg", vec!["-h".to_owned()]).await;
    tokio::pin!(stream);
    let mut actual = false;
    while let Some(Ok(line)) = stream.next().await {
      if line.contains("ffmpeg version") {
        actual = true;
        break;
      }
    }
    assert!(actual)
  }
}

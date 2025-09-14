#[derive(Debug)]
pub struct Task {
  pub words: String,
}

impl Task {
  pub fn greeting(&self) {
    println!("{:?}", self)
  }
}

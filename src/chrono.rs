#[derive(Debug)]
pub struct TimeFormatter {
  hours: usize,
  minutes: usize,
  seconds: usize,
}

impl TimeFormatter {
  pub fn from_seconds(duration_in_secs: f64) -> TimeFormatter {
    let hours = (duration_in_secs / 3600.0).floor() as usize;
    let minutes = ((duration_in_secs % 3600.0) / 60.0).floor() as usize;
    let seconds = (duration_in_secs % 60.0).floor() as usize;
    return TimeFormatter {
      hours: hours,
      minutes: minutes,
      seconds: seconds,
    };
  }

  pub fn from_hhmmss(hh: usize, mm: usize, ss: usize) -> TimeFormatter {
    TimeFormatter {
      hours: hh,
      minutes: mm,
      seconds: ss,
    }
  }

  pub fn to_secs(&self) -> f64 {
    self.hours as f64 * 3600.0
      + self.minutes as f64 * 60.0
      + self.seconds as f64
  }
}

#[cfg(test)]
mod tests {
  use crate::chrono::TimeFormatter;
  #[test]
  fn from_seconds_test() {
    let tf = TimeFormatter::from_seconds(5025.0);
    assert_eq!(tf.hours, 1);
    assert_eq!(tf.minutes, 23);
    assert_eq!(tf.seconds, 45);
  }

  #[test]
  fn to_secs_test() {
    assert_eq!(TimeFormatter::from_hhmmss(1, 23, 45).to_secs(), 5025.0);
  }
}

use chrono::{
  DateTime, Datelike, FixedOffset, NaiveDateTime, TimeZone, Timelike,
};

pub struct SimpleDateTime {
  year: i32,
  month: u32,
  day: u32,
  hour: u32,
  minute: u32,
  second: u32,
}

impl SimpleDateTime {
  pub fn from_str<'a>(
    fmt: &'a str,
    content: &'a str,
    timezone_offset_in_hrs: f32,
  ) -> Option<SimpleDateTime> {
    let naive = NaiveDateTime::parse_from_str(content, fmt).ok()?;
    let offset =
      FixedOffset::east_opt((timezone_offset_in_hrs * 3600.0) as i32)?;
    match offset.from_local_datetime(&naive).single() {
      Some(dt) => Some(SimpleDateTime {
        year: dt.year(),
        month: dt.month(),
        day: dt.day(),
        hour: dt.hour(),
        minute: dt.minute(),
        second: dt.second(),
      }),
      _ => None,
    }
  }
}

#[derive(Debug)]
pub struct TimeFormatter {
  hours: usize,
  minutes: usize,
  seconds: f64,
}

impl TimeFormatter {
  pub fn from_seconds(duration_in_secs: f64) -> TimeFormatter {
    let hours = (duration_in_secs / 3600.0).floor() as usize;
    let minutes = ((duration_in_secs % 3600.0) / 60.0).floor() as usize;
    let seconds = duration_in_secs % 60.0;
    return TimeFormatter {
      hours: hours,
      minutes: minutes,
      seconds: seconds,
    };
  }

  pub fn from_hhmmss(hh: usize, mm: usize, ss: f64) -> TimeFormatter {
    TimeFormatter {
      hours: hh,
      minutes: mm,
      seconds: ss,
    }
  }

  pub fn to_secs(&self) -> f64 {
    self.hours as f64 * 3600.0 + self.minutes as f64 * 60.0 + self.seconds
  }
}

#[cfg(test)]
mod tests {
  use crate::datetime::{SimpleDateTime, TimeFormatter};

  #[test]
  fn parse_datetime() {
    let s = "20260315121253_000010";
    let dt = SimpleDateTime::from_str("%Y%m%d%H%M%S", &s[..14], 8.0)
      .expect("parse failed");
    assert_eq!(dt.year, 2026);
    assert_eq!(dt.month, 3);
    assert_eq!(dt.day, 15);
    assert_eq!(dt.hour, 12);
    assert_eq!(dt.minute, 12);
    assert_eq!(dt.second, 53);
  }

  #[test]
  fn from_seconds_test() {
    {
      let tf = TimeFormatter::from_seconds(5025.0);
      assert_eq!(tf.hours, 1);
      assert_eq!(tf.minutes, 23);
      assert_eq!(tf.seconds, 45.0);
    }
    {
      let tf = TimeFormatter::from_seconds(888.032000);
      assert_eq!(tf.hours, 0);
      assert_eq!(tf.minutes, 14);
      assert!((tf.seconds - 48.032000).abs() < 1e-6);
    }
  }

  #[test]
  fn to_secs_test() {
    assert_eq!(TimeFormatter::from_hhmmss(1, 23, 45.0).to_secs(), 5025.0);
    assert!(
      (TimeFormatter::from_hhmmss(0, 14, 48.032000).to_secs() - 888.032000)
        .abs()
        < 1e-6
    );
  }
}

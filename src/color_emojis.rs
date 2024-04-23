use colored::{ColoredString, Colorize};

pub fn temperature_colored_string(temperature: i32, str: &str) -> ColoredString {
   match temperature {
      ..=-10 => str.bright_blue(),
      -9..=0 => str.bright_blue(),
      1..=10 => str.bright_blue(),
      11..=20 => str.bright_green(),
      21..=30 => str.bright_green(),
      31..=40 => str.bright_purple(),
      41..=50 => str.bright_purple(),
      _ => str.bright_red(),
   }
}

pub fn wind_color(speed: u32, str: String) -> String {
   match speed {
      0..=40 => str,
      41..=60 => str.bright_purple().to_string(),
      61..=100 => str.bright_purple().to_string(),
      _ => str.bright_purple().to_string(),
   }
}

pub fn colored_uv_index_string(uvi: i32, str: &str) -> String {
   match uvi {
      0..=2 => str.to_string(),
      3..=5 => str.bright_purple().to_string(),
      6..=7 => str.bright_purple().to_string(),
      8..=10 => str.bright_purple().to_string(),
      _ => str.bright_purple().to_string(),
   }
}

pub fn humidity_color(humidity: i64, str: String) -> String {
   match humidity {
      0..=30 => str,
      31..=60 => str.bright_yellow().to_string(),
      61..=100 => str.bright_purple().to_string(),
      _ => str.bright_purple().to_string(),
   }
}

pub fn rain_color(rain: i64, str: String) -> String {
   match rain {
      0..=30 => str,
      31..=60 => str.bright_yellow().to_string(),
      61..=100 => str.bright_purple().to_string(),
      _ => str.bright_purple().to_string(),
   }
}

pub fn aqi_color(aqi: i64, str: String) -> String {
   match aqi {
      // 0..=50 => str.bright_green().to_string(),
      0..=50 => str,
      51..=100 => str.bright_yellow().to_string(),
      // 101..=150 => str.truecolor(255, 95, 31).to_string(), // Orange Color
      101..=150 => str.bright_red().to_string(),
      151..=200 => str.bright_red().to_string(),
      201..=300 => str.bright_purple().to_string(),
      _ => str.bright_purple().to_string(),
   }
}

pub fn aqi_emoji(aqi: i64) -> String {
   match aqi {
      0..=50 => "🍃",
      51..=100 => "🍂😷",
      101..=150 => "😷",
      151..=200 => "⚠️😷",
      201..=300 => "🚨😷",
      _ => "😷☠️",
   }
   .to_string()
}

pub fn temperature_feeling(temperature: i32) -> &'static str {
   match temperature {
      ..=-10 => "🥶",
      -9..=0 => "🥶",
      1..=10 => "🥶",
      11..=20 => "😊",
      21..=30 => "😊",
      31..=40 => "🥵",
      41..=50 => "🔥",
      _ => "🔥",
   }
}
   
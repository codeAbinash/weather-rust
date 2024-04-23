pub const USER_NAME: &'static str = "Abinash";
pub const DEFAULT_LOCATION: &'static str = "Bankura";
pub const DEFAULT_LAT: f64 = 23.2325;
pub const DEFAULT_LON: f64 = 87.0654;
pub const DEFAULT_API_KEY: &'static str = "0e376e0750966cdba160fc85a4bb0427";
pub const VERSION: &'static str = "0.0.1-beta.1";
pub const REPO_LINK: &'static str = "https://github.com/codeAbinash/weather-rs/";

use chrono::prelude::*;

pub fn k_to_c(k: f64) -> i32 {
   (k - 273.15).round() as i32
}

// 2 digit precision
pub fn to_2_digit_precision(num: f64) -> f64 {
   (num * 100.0).round() / 100.0
}

pub fn capitalize(s: &str) -> String {
   s.split(' ')
      .map(|word| {
         let mut chars = word.chars();
         match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().chain(chars).collect(),
         }
      })
      .collect::<Vec<_>>()
      .join(" ")
}

pub fn greeting_string(hour: u32) -> String {
   let greeting = match hour {
      0..=3 => "Good Night 🌓",
      4..=5 => "Good Morning 🌅",
      6..=7 => "Good Morning 🌄",
      8..=9 => "Good Morning 🌻",
      10..=11 => "Good Morning 🌞",
      12..=13 => "Good Noon ⛱️",
      14..=15 => "Good Noon ☀️",
      16..=17 => "Good Afternoon 🌇",
      18..=19 => "Good Evening 🌆",
      20..=21 => "Good Night 🌖",
      22..=23 => "Good Night 🌔",
      _ => "Good Night 🌃",
   };

   greeting.to_string()
}

pub fn get_formatted_time(time: i64) -> String {
   let dt = Local.timestamp_opt(time, 0).unwrap();
   dt.format("%-I:%M %p").to_string()
}

pub fn split_to_chars_newline(s: &str, n: usize) -> String {
   s.split_whitespace()
      .fold((String::new(), 0), |(mut s, mut l), word| {
         if l + word.len() > n {
            s.push('\n');
            l = 0;
         }
         if l > 0 {
            s.push(' ');
            l += 1;
         }
         s.push_str(word);
         l += word.len();
         (s, l)
      })
      .0
}

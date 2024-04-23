use crate::color_emojis::*;
use weather::*;

pub fn status_string(s: &str) -> String {
   let mut status = format!("☁️ {}", s);
   status = capitalize(&status);
   // Make the length of the status string to be 20
   status = format!("{:20}", status);
   status
}

pub fn feels_like_string(feels_like: f64) -> String {
   return format!(
      "{} Feels Like: {}",
      temperature_feeling(k_to_c(feels_like)),
      temperature_colored_string(
         k_to_c(feels_like),
         format!("{}°C", k_to_c(feels_like)).as_str()
      )
   );
}

pub fn temp_string(temp: f64, max: f64, min: f64) -> String {
   return format!(
      "🌡️ {} {} {}°/{}°C",
      temperature_colored_string(k_to_c(temp), format!("{}°C", k_to_c(temp)).as_str()),
      temperature_feeling(k_to_c(temp)),
      k_to_c(max),
      k_to_c(min),
   );
}

pub fn wind_string(speed: f64, deg: i64) -> String {
   let str = format!("🌪️ Wind: {}m/s {}°", to_2_digit_precision(speed), deg);
   return wind_color(speed as u32, str);
}

pub fn uv_index_string(uvi: f64) -> String {
   // return format!("🌞 {} UV index", uvi);
   return format!(
      "🌞 {}",
      colored_uv_index_string(
         uvi as i32,
         format!("UV index: {} {}", uvi, uv_index_emoji(uvi as i32)).as_str(),
      ),
   );
}

fn uv_index_emoji(uvi: i32) -> &'static str {
   match uvi {
      0..=2 => "",
      _ => "⚠️",
   }
}

pub fn humidity_string(humidity: i64) -> String {
   return format!(
      "🌫️ {}",
      humidity_color(humidity, format!("Humidity: {}%", humidity))
   );
}

pub fn cloud_string(cloud: i64) -> String {
   return format!("☁️ Cloud: {}% Covered", cloud);
}

pub fn pressure_string(pressure: i64) -> String {
   return format!("🌬️ Pressure: {} hPa", pressure);
}

pub fn rain_string(rain: Option<f64>) -> String {
   match rain {
      Some(rain) => {
         let rain = (rain * 100.0) as i64;
         rain_color(rain, format!("🌧️ Rain Chance: {}% ", rain))
      }
      None => format!("🌧️ --% chance of rain",),
   }
}

pub fn aqi_string(aqi: i64) -> String {
   let aqi_str = format!("{:20}", format!("🍃 AQI: {} {}", aqi, aqi_emoji(aqi)));
   return aqi_color(aqi, aqi_str);
}

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Weather {
   lat: f64,
   lon: f64,
   timezone: String,
   timezone_offset: i64,
   pub current: Current,
   pub hourly: Vec<Current>,
   pub daily: Vec<Daily>,
   pub alerts: Option<Vec<Alert>>,
}

#[derive(Serialize, Deserialize)]
pub struct Alert {
   sender_name: String,
   pub event: String,
   start: i64,
   end: i64,
   pub description: String,
   pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Current {
   pub dt: i64,
   pub sunrise: Option<i64>,
   pub sunset: Option<i64>,
   pub temp: f64,
   pub feels_like: f64,
   pub pressure: i64,
   pub humidity: i64,
   pub dew_point: f64,
   pub uvi: f64,
   pub clouds: i64,
   pub visibility: i64,
   pub wind_speed: f64,
   pub wind_deg: i64,
   pub wind_gust: f64,
   pub weather: Vec<WeatherElement>,
   pub pop: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct WeatherElement {
   id: i64,
   pub main: String,
   pub description: String,
   icon: String,
}

#[derive(Serialize, Deserialize)]
pub struct Daily {
   pub dt: i64,
   pub sunrise: i64,
   pub sunset: i64,
   pub moonrise: i64,
   pub moonset: i64,
   pub moon_phase: f64,
   pub temp: Temp,
   pub feels_like: FeelsLike,
   pub pressure: i64,
   pub humidity: i64,
   pub dew_point: f64,
   pub wind_speed: f64,
   pub wind_deg: i64,
   pub wind_gust: f64,
   pub weather: Vec<WeatherElement>,
   pub clouds: i64,
   pub pop: f64,
   pub uvi: f64,
}

#[derive(Serialize, Deserialize)]
pub struct FeelsLike {
   day: f64,
   night: f64,
   eve: f64,
   morn: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Temp {
   pub day: f64,
   pub min: f64,
   pub max: f64,
   pub night: f64,
   pub eve: f64,
   pub morn: f64,
}

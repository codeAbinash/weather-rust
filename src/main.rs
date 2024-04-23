use colored::Colorize;
use commands::execute_command;
use dotenv::dotenv;
use reqwest::Client;
use std::env;
use strings::*;
use types::{
   aqi::Aqi,
   weather::{Alert, Weather},
};
use weather::*;

pub mod color_emojis;
pub mod commands;
pub mod strings;
pub mod types;

async fn fetch_url(weather_url: &str, aqi_url: &str, location_name: &str) {
   let client = Client::new();

   let weather_future = client.get(weather_url).send();
   let aqi_future = client.get(aqi_url).send();

   let (weather_response, aqi_response) = tokio::try_join!(weather_future, aqi_future).unwrap();

   let weather = weather_response.json::<Weather>().await;
   let aqi = aqi_response.json::<Aqi>().await;

   // Read JSON data from files local/res.json and local/aqi.json - Local for testing
   // let data = fs::read_to_string("local/res.json").expect("Unable to read file");
   // let weather = serde_json::from_str::<Weather>(&data);

   // let aqi = fs::read_to_string("local/aqi.json").expect("Unable to read file");
   // let aqi = serde_json::from_str::<aqi_types::Aqi>(&aqi);

   match weather {
      Ok(weather) => match aqi {
         Ok(aqi) => {
            print_weather(weather, aqi, location_name);
         }
         Err(e) => {
            println!("Error: {}", e);
         }
      },
      Err(e) => {
         println!("Error: {}", e);
      }
   }
}

fn print_weather(weather: Weather, aqi: Aqi, location_name: &str) {
   let w = weather.current;
   let h = weather.hourly;
   let alerts = weather.alerts;
   let d = weather.daily;

   let sunrise = get_formatted_time(w.sunrise.unwrap());
   let sunset = get_formatted_time(w.sunset.unwrap());
   let status = status_string(&w.weather[0].description);
   let temp = temp_string(w.temp, d[0].temp.max, d[0].temp.min);
   let feels_like = feels_like_string(w.feels_like);
   let uv_index = uv_index_string(w.uvi);
   let humidity = humidity_string(w.humidity);
   let wind = wind_string(w.wind_speed, w.wind_deg);
   let cloud = cloud_string(w.clouds);
   let sunrise = format!("🌅 Sunrise {}", sunrise);
   let sunset = format!("🌇 Sunset  {}", sunset);
   let pressure = pressure_string(w.pressure);
   let air_quality = aqi_string(aqi.list[0].main.aqi);
   let rain = rain_string(h[0].pop);

   // println!("Hello {USER_NAME} 👋🏻{}", greeting_string(hour));
   println!("\nCurrent Weather ☁️ Report - 🗺️ {location_name}");
   println!("{}", "---------------------------------------------");
   println!("{} \t{}", status, uv_index);
   println!("{} \t{}", temp, wind);
   println!("{} \t{}", feels_like, cloud);
   println!("{} \t{}", humidity, pressure);
   println!("{} \t{}", air_quality, sunrise);
   println!("{} \t{}", rain, sunset);
   println!("{}", "---------------------------------------------");

   print_alerts(alerts);

   // println!("{}", "Have a nice day! 😊 🌈\n");
}

fn print_alerts(alerts: Option<Vec<Alert>>) {
   if let Some(alerts) = alerts {
      for alert in alerts {
         let alert_str =
            format!("Alert: {} [{}]", alert.event, alert.tags.join(", ")).bright_purple();
         println!("{}\n", alert_str);
      }
   }
}

fn generate_urls(lat: f64, lon: f64, api_key: &str) -> (String, String) {
   let weather_url = format!(
      "https://api.openweathermap.org/data/2.5/onecall?lat={}&lon={}&exclude=minutely&appid={}",
      lat, lon, api_key
   );
   let aqi_url = format!(
      "https://api.openweathermap.org/data/2.5/air_pollution?lat={}&lon={}&appid={}",
      lat, lon, api_key
   );
   (weather_url, aqi_url)
}

fn weather() {
   dotenv().ok();
   let lat: f64 = env::var("LATITUDE")
      .unwrap_or_else(|_| DEFAULT_LAT.to_string())
      .parse()
      .unwrap();
   let lon: f64 = env::var("LONGITUDE")
      .unwrap_or_else(|_| DEFAULT_LON.to_string())
      .parse()
      .unwrap();
   let api_key = env::var("API_KEY").unwrap_or_else(|_| DEFAULT_API_KEY.to_string());

   let location_name = env::var("LOCATION_NAME").unwrap_or_else(|_| DEFAULT_LOCATION.to_string());
   let (weather_url, aqi_url) = generate_urls(lat, lon, &api_key);
   let rt = tokio::runtime::Runtime::new().unwrap();
   rt.block_on(fetch_url(&weather_url, &aqi_url, location_name.as_str()));
}

fn main() {
   let args: Vec<String> = env::args().collect();

   if args.len() > 1 {
      execute_command(&args[1])
   } else {
      weather();
   }
}

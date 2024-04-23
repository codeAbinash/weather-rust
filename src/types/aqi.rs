use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Aqi {
   coord: Coord,
   pub list: Vec<List>,
}

#[derive(Serialize, Deserialize)]
pub struct Coord {
   lon: f64,
   lat: f64,
}

#[derive(Serialize, Deserialize)]
pub struct List {
   pub main: Main,
   components: HashMap<String, f64>,
   dt: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Main {
   pub aqi: i64,
}

/* coding: utf-8 */
/******************************************************************************
 * weather_exporter
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/
use std::time::Duration;
use std::sync::Arc;

use serde::{Serialize, Deserialize};
use serde::export::Formatter;
use string_error::static_err;

use crate::context::Context;

pub async fn fetch_all(ctx: Arc<Context>) -> Result<Vec<Response>, Box<dyn std::error::Error>>
{
  let mut result = Vec::new();
  for location in ctx.locations.iter() {
    result.push(fetch(ctx.clone(), location.clone()).await?);
  }
  Ok(result)
}

#[derive(Debug)]
pub struct FetchError {
  location: String,
  err: Box<dyn std::error::Error>,
  original: String,
}

impl std::error::Error for FetchError {

}

impl std::fmt::Display for FetchError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Failed to fetch {}:\n {}\n---\n{}", self.location, self.err, self.original)
  }
}

pub async fn fetch(ctx: Arc<Context>, location: String) -> Result<Response, Box<dyn std::error::Error>> {
  if let Some(resp) = ctx.cache.read().unwrap().get(&location) {
    return Ok(resp.clone());
  }
  let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", location, ctx.app_id);
  let body = reqwest::get(url.as_str())
    .await?
    .text()
    .await?;
  let resp = serde_json::from_str::<Response>(body.as_str())
    .map_err(|err| {
      let err = FetchError {
        location: location.to_string(),
        err: Box::new(err),
        original: body,
      };
      Box::new(err)
    })?;
  ctx.cache.write().unwrap()
    .insert_with_ttl(location, resp.clone(), Duration::from_secs(15))
    .map(|_| resp)
    .map_err(|_| static_err("Failed to write to cache"))
}

fn nan64() -> f64 {
  std::f64::NAN
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coord {
  pub lon: f64,
  pub lat: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weather {
  pub id: u32,
  pub main: String,
  pub description: String,
  pub icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Main {
  pub temp: f64,
  #[serde(default = "nan64")]
  pub feels_like: f64,
  pub temp_min: f64,
  pub temp_max: f64,
  pub pressure: f64,
  pub humidity: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wind {
  pub speed: f64,
  pub deg: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Clouds {
  pub all: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rain {
  #[serde(default, rename = "1h")]
  pub value_1h: Option<f64>,
  #[serde(default, rename = "3h")]
  pub value_3h: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snow {
  #[serde(default, rename = "1h")]
  pub value_1h: Option<f64>,
  #[serde(default, rename = "3h")]
  pub value_3h: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sys {
  #[serde(default, rename = "type")]
  type_no: u32,
  #[serde(default)]
  id: u32,
  #[serde(default)]
  message: f32,
  pub country: String,
  pub sunrise: u64,
  pub sunset: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
  pub coord: Coord,
  pub weather: Vec<Weather>,
  base: String,
  pub main: Main,
  pub visibility: f64,
  pub wind: Wind,
  pub clouds: Clouds,
  #[serde(default)]
  pub rain: Option<Rain>,
  #[serde(default)]
  pub snow: Option<Rain>,
  #[serde(default)]
  pub dt: u64,
  pub sys: Sys,
  #[serde(default)]
  pub timezone: i32,
  pub id: u32,
  pub name: String,
  cod: u32,
}


#[test]
fn parse_sample() -> serde_json::Result<()> {
  let response: Response = serde_json::from_str(TEST_DATA)?;
  assert_eq!(response.main.temp, 280.32);
  serde_json::Result::Ok(())
}

#[cfg(test)]
const TEST_DATA: &'static str = r##"
{
  "coord":{
    "lon":-0.13,
    "lat":51.51
  },
  "weather":[
    {
      "id":300,
      "main":"Drizzle",
      "description":"light intensity drizzle",
      "icon":"09d"
    }
  ],
  "base":"stations",
  "main":{
    "temp":280.32,
    "pressure":1012,
    "humidity":81,
    "temp_min":279.15,
    "temp_max":281.15
  },
  "visibility":10000,
  "wind":{
    "speed":4.1,
    "deg":80
  },
  "clouds":{
    "all":90
  },
  "dt":1485789600,
  "sys":{
    "type":1,
    "id":5091,
    "message":0.0103,
    "country":"GB",
    "sunrise":1485762037,
    "sunset":1485794875
  },
  "id":2643743,
  "name":"London",
  "cod":200
}
"##;

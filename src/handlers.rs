/* coding: utf-8 */
/******************************************************************************
 * weather_exporter
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/

extern crate warp;

use std::str::FromStr;

use warp::reply::Reply;
use warp::reject;
use warp::http::uri;

use std::sync::RwLock;
use once_cell::sync::Lazy;
use config::Config;

use crate::api;

static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| RwLock::new(Config::default()));

pub fn set_locations(locations: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
  CONFIG
    .write().expect("cannot lock")
    .set::<Vec<String>>("locations", locations)
    .map(|_| ())
    .map_err(|err| err.into())
}

fn get_locations() -> Vec<String> {
  CONFIG
    .read().expect("cannot lock")
    .get_array("locations").unwrap_or(vec![])
    .iter().map(|v| v.to_string()).collect()
}

fn render(resp: &api::Response) -> String {
  format!(r####"
### {location}

## Air

#HELP wether_air_temp Air temperature in Kelvin.
#TYPE wether_air_temp guage
wether_air_temp{{planet="Earth", location="{location}"}} {temp}

#HELP wether_air_humidity Air himidity in percentage.
#TYPE wether_air_humidity guage
wether_air_humidity{{planet="Earth", location="{location}"}} {humidity}

#HELP sensor_air_pressure Air pressure in hectopascals (hPa)
#TYPE sensor_air_pressure guage
wether_air_pressure{{planet="Earth", location="{location}"}} {pressure}

## Cloud

#HELP wether_clouds Cloudiness in percentage.
#TYPE wether_clouds guage
wether_clouds{{planet="Earth", location="{location}"}} {clouds}

## Wind

#HELP wether_wind_speed Wind speed. meter/sec.
#TYPE wether_wind_speed guage
wether_wind_speed{{planet="Earth", location="{location}"}} {wind_speed}
#HELP wether_wind_deg Wind direction, degrees (meteorological).
#TYPE wether_wind_deg guage
wether_wind_deg{{planet="Earth", location="{location}"}} {wind_deg}

## Sun

#HELP wether_sunrise Sunrise time, unix, UTC
#TYPE wether_sunrise guage
wether_sunrise{{planet="Earth", location="{location}"}} {sunrise}
#HELP wether_sunset Sunset time, unix, UTC
#TYPE wether_sunset guage
wether_sunset{{planet="Earth", location="{location}"}} {sunset}
"####,
          location=resp.name,
          temp=resp.main.temp,
          humidity=resp.main.humidity,
          pressure=resp.main.pressure,
          clouds=resp.clouds.all,
          wind_speed=resp.wind.speed,
          wind_deg=resp.wind.deg,
          sunrise=resp.sys.sunrise,
          sunset=resp.sys.sunset,
  )
}

pub async fn index() -> Result<impl Reply, reject::Rejection> {
  let result = api::fetch_all(&get_locations()).await;
  match result {
    Ok(responses) => {
      let resp: Vec<String> = responses.iter().map(render).collect();
      let body = resp.join("\n");
      let resp = warp::http::Response::builder()
        .status(200)
        .header("Content-Type", "text/plain;charset=UTF-8")
        .body(body)
        .unwrap();
      Ok(resp)
    },
    Err(err) => {
      let resp = warp::http::Response::builder()
        .status(502)
        .header("Content-Type", "text/plain;charset=UTF-8")
        .body(err.to_string())
        .unwrap();
      Ok(resp)
    },
  }
}

pub async fn not_found() -> Result<impl Reply, reject::Rejection> {
  Ok(warp::redirect(uri::Uri::from_str("/").unwrap()))
}
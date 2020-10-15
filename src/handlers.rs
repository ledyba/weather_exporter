/* coding: utf-8 */
/******************************************************************************
 * weather_exporter
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/

use log::{info};

extern crate serde;
use serde::Deserialize;

extern crate warp;

use warp::reply::Reply;
use warp::reject;

use crate::api;
use crate::context::Context;
use std::sync::Arc;

fn render(resp: &api::Response) -> String {
  let mut body = format!(r####"
### {location}

## Air

#HELP weather_air_temp Air temperature in Kelvin.
#TYPE weather_air_temp guage
weather_air_temp{{planet="Earth", location="{location}"}} {temp}

#HELP weather_air_humidity Air himidity in percentage.
#TYPE weather_air_humidity guage
weather_air_humidity{{planet="Earth", location="{location}"}} {humidity}

#HELP sensor_air_pressure Air pressure in hectopascals (hPa)
#TYPE sensor_air_pressure guage
weather_air_pressure{{planet="Earth", location="{location}"}} {pressure}

## Cloud

#HELP weather_clouds Cloudiness in percentage.
#TYPE weather_clouds guage
weather_clouds{{planet="Earth", location="{location}"}} {clouds}

## Wind

#HELP weather_wind_speed Wind speed. meter/sec.
#TYPE weather_wind_speed guage
weather_wind_speed{{planet="Earth", location="{location}"}} {wind_speed}
#HELP weather_wind_deg Wind direction, degrees (meteorological).
#TYPE weather_wind_deg guage
weather_wind_deg{{planet="Earth", location="{location}"}} {wind_deg}

## Sun

#HELP weather_sunrise Sunrise time, unix, UTC
#TYPE weather_sunrise guage
weather_sunrise{{planet="Earth", location="{location}"}} {sunrise}
#HELP weather_sunset Sunset time, unix, UTC
#TYPE weather_sunset guage
weather_sunset{{planet="Earth", location="{location}"}} {sunset}
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
  );
  if let Some(rain) = &resp.rain {
    if let Some(value) = rain.value_1h {
      body = format!(r####"
{body}
#HELP weather_rain_1h Rain volume for the last 1 hour, mm
#TYPE weather_rain_1h guage
weather_rain_1h{{planet="Earth", location="{location}"}} {value}
"####,
                     body=body,
                     location=resp.name,
                     value=value)
    }
    if let Some(value) = rain.value_3h {
      body = format!(r####"
{body}
#HELP weather_rain_3h Rain volume for the last 3 hour, mm
#TYPE weather_rain_3h guage
weather_rain_3h{{planet="Earth", location="{location}"}} {value}
"####,
                     body=body,
                     location=resp.name,
                     value=value)
    }
  }
  if let Some(snow) = &resp.snow {
    if let Some(value) = snow.value_1h {
      body = format!(r####"
{body}
#HELP weather_snow_1h Snow volume for the last 1 hour, mm
#TYPE weather_snow_1h guage
weather_snow_1h{{planet="Earth", location="{location}"}} {value}
"####,
                     body=body,
                     location=resp.name,
                     value=value)
    }
    if let Some(value) = snow.value_3h {
      body = format!(r####"
{body}
#HELP weather_snow_3h Snow volume for the last 3 hour, mm
#TYPE weather_snow_3h guage
weather_snow_3h{{planet="Earth", location="{location}"}} {value}
"####,
                     body=body,
                     location=resp.name,
                     value=value)
    }
  }
  body
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeParams {
  #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
  pub location: Option<String>,
  #[serde(rename = "app-id", skip_serializing_if = "Option::is_none")]
  pub app_id: Option<String>,
}

pub async fn index() -> Result<impl Reply, reject::Rejection>
{
  let resp = warp::http::Response::builder()
    .status(500)
    .header("Content-Type", "text/plain;charset=UTF-8")
    .body("Weather Exporter")
    .unwrap();
  Ok(resp)
}

pub async fn probe(ctx: Arc<Context>, params: ProbeParams) -> Result<impl Reply, reject::Rejection>
{
  let app_id  = if params.app_id.is_some() {
    params.app_id.unwrap().clone()
  } else {
    ctx.app_id.clone()
  };
  if app_id.is_empty() {
    let resp = warp::http::Response::builder()
      .status(402)
      .header("Content-Type", "text/plain;charset=UTF-8")
      .body("app id is not given!".to_string())
      .unwrap();
    return Ok(resp);
  }
  if params.location.is_none() {
    let resp = warp::http::Response::builder()
      .status(404)
      .header("Content-Type", "text/plain;charset=UTF-8")
      .body("Empty location is not found, by definition...".to_string())
      .unwrap();
    return Ok(resp);
  }
  let location = params.location.unwrap().clone();
  info!("Probe: app-id={} location={}", app_id.as_str(), location.as_str());
  let result = api::fetch(ctx.clone(), app_id, location).await;
  match result {
    Ok(responses) => {
      let body = render(&responses);
      let resp = warp::http::Response::builder()
        .status(200)
        .header("Content-Type", "text/plain;charset=UTF-8")
        .body(body)
        .unwrap();
      Ok(resp)
    },
    Err(err) => {
      let resp = warp::http::Response::builder()
        .status(500)
        .header("Content-Type", "text/plain;charset=UTF-8")
        .body(err.to_string())
        .unwrap();
      Ok(resp)
    },
  }
}

pub async fn bad_request() -> Result<impl Reply, reject::Rejection> {
  let resp = warp::http::Response::builder()
    .status(400)
    .header("Content-Type", "text/plain;charset=UTF-8")
    .body("Invalid request".to_string())
    .unwrap();
  Ok(resp)
}

pub async fn not_found() -> Result<impl Reply, reject::Rejection> {
  let resp = warp::http::Response::builder()
    .status(404)
    .header("Content-Type", "text/plain;charset=UTF-8")
    .body("Endpoint not found".to_string())
    .unwrap();
  Ok(resp)
}
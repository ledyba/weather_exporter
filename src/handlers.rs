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

use crate::api;
use crate::config::Config;
use std::sync::Arc;

fn render(resp: &api::Response) -> String {
  format!(r####"
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
  )
}

pub async fn index(conf: Arc<Config>) -> Result<impl Reply, reject::Rejection>
{
  let result = api::fetch_all(&conf.app_id, &conf.locations).await;
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
/* coding: utf-8 */
/******************************************************************************
 * weather_exporter
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/

#[derive(Debug, Clone)]
pub struct Config {
  pub app_id: String,
  pub locations: Vec<String>,
}
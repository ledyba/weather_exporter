/* coding: utf-8 */
/******************************************************************************
 * weather_exporter
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/

use crate::api::Response;
use std::sync::RwLock;

pub struct Context {
  pub app_id: String,
  pub cache: RwLock<cascara::Cache<String, Response>>,
}

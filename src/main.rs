/* coding: utf-8 */
/******************************************************************************
 * weather_exporter
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/
use std::process::exit;

use log::{error};
use clap::{App, Arg, SubCommand, ArgMatches};
use warp::Filter;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

mod api;
mod handlers;
mod context;

fn web(m: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
  let sock = if let Some(listen) = m.value_of("listen") {
    std::net::SocketAddr::from_str(listen)?
  } else {
    return Err("listen is not set.".into())
  };

  let ctx = Arc::new(context::Context {
    app_id: m.value_of("app_id").unwrap_or("").to_string(),
    cache: RwLock::new(cascara::Cache::with_window_size(100, 20)),
  });

  let cores = num_cpus::get();

  let mut rt = tokio::runtime::Builder::new()
    .core_threads(cores + 1)
    .threaded_scheduler()
    .enable_all()
    .build()
    .unwrap();

  rt.block_on(async move {
    let probe = warp::path::path("probe").and(
      warp::any()
        .and(warp::path::end())
        .and(warp::query())
        .and_then({
          let ctx = ctx.clone();
          move |p: handlers::ProbeParams| handlers::probe(ctx.clone(), p)
        })
        .or(warp::any().and_then(handlers::bad_request)));
    let index = warp::path::end().and_then(handlers::index);
    let router = probe.or(index).or(warp::any().and_then(handlers::not_found));
    warp::serve(router)
      .run(sock)
      .await;
  });
  Ok(())
}

fn main() {
  env_logger::init_from_env(
    env_logger::Env::from(env_logger::Env::default())
      .default_filter_or("info"));
  let app = App::new("weather_exporter")
    .version("0.1.0")
    .author("Kaede Fujisaki <psi@7io.org>")
    .about("Monitor weathers in prometheus!")
    .subcommand(SubCommand::with_name("web")
      .arg(Arg::with_name("listen")
        .long("listen")
        .takes_value(true)
        .allow_hyphen_values(true)
        .default_value("0.0.0.0:8080")
        .required(false))
      .arg(Arg::with_name("app_id")
        .long("app-id")
        .takes_value(true)
        .allow_hyphen_values(true)
        .required(false)));
  let m = app.get_matches();
  match m.subcommand_name() {
    Some("web") => {
      if let Err(err) = web(m.subcommand_matches("web").unwrap()) {
        error!("Failed to start web: {:?}\n", err);
        exit(-1);
      }
    }
    None | Some(_) => {
      error!("{}\n", m.usage());
      exit(-1);
    }
  }
}

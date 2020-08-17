/* coding: utf-8 */
/******************************************************************************
 * weather_exporter
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/
use std::process::exit;

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

  let cfg = Arc::new(context::Context {
    app_id: m.value_of("app_id").expect("app-id is not set.").to_string(),
    locations: m.values_of("LOCATIONS").expect("location is not set.").map(|s| s.to_string()).collect(),
    cache: RwLock::new(cascara::Cache::with_window_size(100, 20)),
  });

  let mut rt = tokio::runtime::Builder::new()
    .core_threads(32)
    .threaded_scheduler()
    .enable_all()
    .build()
    .unwrap();

  rt.block_on(async move {
    let index = move || handlers::index(cfg.clone());
    let router = warp::path::end().and_then(index)
      .or(warp::any().and_then(handlers::not_found));
    warp::serve(router)
      .run(sock)
      .await;
  });
  Ok(())
}

fn main() {
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
        .required(true))
      .arg(Arg::with_name("LOCATIONS")
        .help("Name of locations. e.g.) Tokyo, London")
        .index(1)
        .takes_value(true)
        .multiple(true)
        .required(true)));
  let m = app.get_matches();
  match m.subcommand_name() {
    Some("web") => {
      if let Err(err) = web(m.subcommand_matches("web").unwrap()) {
        eprint!("Failed to start web: {:?}\n", err);
        exit(-1);
      }
    }
    None | Some(_) => {
      eprint!("{}\n", m.usage());
      exit(-1);
    }
  }
}

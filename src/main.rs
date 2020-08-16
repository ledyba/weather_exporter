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

mod api;
mod handlers;

fn web(m: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
  let sock = if let Some(listen) = m.value_of("listen") {
    std::net::SocketAddr::from_str(listen)?
  } else {
    return Err("listen is not set.".into())
  };

  if let Some(app_id) = m.value_of("app_id") {
    api::set_app_id(app_id)?;
  } else {
    return Err("app-id is not set.".into())
  }

  if let Some(locations) = m.values_of("LOCATIONS") {
    handlers::set_locations(locations.map(|s| s.to_string()).collect())?;
  } else {
    return Err("app-id is not set.".into())
  }

  let mut rt = tokio::runtime::Builder::new()
    .core_threads(32)
    .threaded_scheduler()
    .enable_all()
    .build()
    .unwrap();

  let router = warp::path::end().and_then(handlers::index)
    .or(warp::any().and_then(handlers::not_found));

  rt.block_on(async {
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

// Copyright (C) 2017-2025 Smart Code OOD 203358507

#![cfg_attr(
    all(target_os = "windows", feature = "bundled"),
    windows_subsystem = "windows"
)]
use std::error::Error;

use clap::Parser;
use env_logger::Env;

use stremio_service::app::{handle_stremio_protocol, Application};
use stremio_service::args::Args;
use stremio_service::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // parse args before env_logger init to set the appropriate log level for native messaging
    let args = Args::parse();

    // the native messaging web extension will error if any output is >= 1 MB
    env_logger::Builder::from_env(Env::default().default_filter_or(match args.addon_id {
        Some(_) => "off",
        None => "info"
    })).init();

    if let Some(url) = args.open.as_ref() {
        if !url.is_empty() {
            handle_stremio_protocol(url.clone());
        }
    }

    let config = Config::new(args)?;
    log::info!("Using service configuration: {:#?}", config);

    let application = Application::new(config);

    Ok(application.run().await?)
}

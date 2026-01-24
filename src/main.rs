use std::fs;

use clap::Parser;
use color_eyre::eyre::Result;
use reqwest::header;
use serde::Deserialize;

mod api;

#[derive(Debug, Parser)]
struct Args {
    #[clap(long, short, default_value = "config.toml")]
    config: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    token: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let config: Config = toml::from_slice(&fs::read(args.config)?)?;

    let mut token_value = header::HeaderValue::from_str(&config.token)?;
    token_value.set_sensitive(true);

    let mut headers = header::HeaderMap::new();
    headers.insert("Open-Shock-Token", token_value);

    let client = reqwest::blocking::ClientBuilder::new()
        .default_headers(headers)
        .user_agent("OpenShock-TUI")
        .build()?;

    let resp = client
        .get("https://api.openshock.app/1/shockers/logs")
        .send()?
        .error_for_status()?;

    let log_data: api::LogResponse = resp.json()?;
    println!("{:#?}", log_data);

    Ok(())
}

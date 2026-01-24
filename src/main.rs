use std::{fs, time::Duration};

use clap::Parser;
use color_eyre::eyre::Result;
use jiff::{Timestamp, Unit};
use ratatui::crossterm::event::{self, Event, KeyCode};
use reqwest::header;
use serde::Deserialize;

use crate::{
    api::shocker_logs,
    ui::{Message, RunningState, ViewModel, update, view},
};

mod api;
mod ui;

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

    let mut terminal = ratatui::init();

    let mut model = ViewModel {
        log: Vec::new(),
        last_updated: Timestamp::from_second(0)?,
        running_state: RunningState::Running,
    };

    let app_result = loop {
        terminal.draw(|f| view(&model, f))?;

        if let Some(msg) = handle_event(&model)? {
            model = update(model, msg);
        }

        if Timestamp::now()
            .since(model.last_updated)?
            .total(Unit::Second)?
            > 5.0
        {
            model = update(model, Message::UpdateLog(shocker_logs(&client)?));
        }

        if model.running_state == RunningState::Done {
            break Ok(());
        }
    };
    ratatui::restore();

    app_result
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}

fn handle_event(model: &ViewModel) -> Result<Option<Message>> {
    if event::poll(Duration::from_secs(1))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }

    Ok(None)
}

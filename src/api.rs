use color_eyre::eyre::Result;
use jiff::Timestamp;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct LogResponse {
    pub(crate) logs: Vec<LogEntry>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LogEntry {
    pub(crate) id: String,
    #[serde(rename = "hubId")]
    pub(crate) hub_id: String,
    #[serde(rename = "hubName")]
    pub(crate) hub_name: String,
    #[serde(rename = "shockerId")]
    pub(crate) shocker_id: String,
    #[serde(rename = "shockerName")]
    pub(crate) shocker_name: String,
    #[serde(rename = "createdOn")]
    pub(crate) created_on: Timestamp,
    #[serde(rename = "type")]
    pub(crate) typ: String,
    #[serde(rename = "controlledBy")]
    pub(crate) controlled_by: ControlledBy,
    pub(crate) intensity: u32,
    pub(crate) duration: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ControlledBy {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) image: String,
    #[serde(rename = "customName")]
    pub(crate) custom_name: Option<String>,
}

pub(crate) fn shocker_logs(client: &Client) -> Result<LogResponse> {
    let resp = client
        .get("https://api.openshock.app/1/shockers/logs")
        .send()?
        .error_for_status()?;

    Ok(resp.json()?)
}

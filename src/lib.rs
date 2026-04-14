#![allow(dead_code)]

use std::time::Duration;

use anyhow::Result;
use ureq::Agent;

use crate::{config::Config, domain::Weather, waybar::Waybar};

pub mod config;

mod domain;
mod icons;
mod waybar;

pub fn weather_to_waybar(config: Config) -> Result<String> {
    let agent: Agent = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(2)))
        .build()
        .into();

    let city = config.city.unwrap_or("".to_string());
    let url = format!("https://wttr.in/{}", city);
    let raw = agent
        .get(url)
        .query("format", "j1")
        .call()?
        .body_mut()
        .read_to_string()?;

    // TODO: save to cache here

    let weather: Weather = serde_json::from_str(&raw)?;
    let waybar = Waybar::from(weather, config.unit);

    Ok(serde_json::to_string(&waybar)?)
}

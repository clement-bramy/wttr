use serde::Serialize;
use wttr::weather_for_city;

use crate::icons::icon_from_id;

mod icons;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weather = weather_for_city("Sydney")?;
    let output = Output::new(
        icon_from_id(weather.code()),
        weather.feels(),
        &weather.tooltip(),
    );
    let output = serde_json::to_string(&output)?;
    print!("{}", output);

    Ok(())
}

#[derive(Serialize)]
struct Output {
    text: String,
    tooltip: String,
}

impl Output {
    pub fn new(icon: &str, temperature: &str, tooltip: &str) -> Self {
        Self {
            text: format!("{} {}°C", icon, temperature),
            tooltip: tooltip.to_string(),
        }
    }
}

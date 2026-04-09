#![allow(dead_code)]

use serde::{Deserialize, Deserializer};
use ureq::get;

use crate::icons::icon_from_id;

mod icons;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize)]
pub struct CurrentCondition {
    #[serde(rename = "weatherCode")]
    weather_code: String,

    #[serde(rename = "FeelsLikeC")]
    feels_like_c: String,

    #[serde(rename = "temp_C")]
    temp_c: String,

    #[serde(rename = "weatherDesc", deserialize_with = "first_value")]
    description: String,

    #[serde(rename = "windspeedKmph")]
    wind_speed: String,

    humidity: String,
}

#[derive(Debug, Deserialize)]
struct Day {
    date: String,

    #[serde(rename = "maxtempC")]
    max_temp_c: String,

    #[serde(rename = "mintempC")]
    min_temp_c: String,

    #[serde(deserialize_with = "first")]
    astronomy: Astronomy,

    #[serde(rename = "hourly")]
    changes: Vec<HourlyChange>,
}

#[derive(Debug, Deserialize)]
struct Astronomy {
    sunrise: String,
    sunset: String,
}

#[derive(Debug, Deserialize)]
struct HourlyChange {
    time: String,

    #[serde(rename = "weatherCode")]
    code: String,

    #[serde(rename = "FeelsLikeC")]
    fells_like_c: String,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    #[serde(deserialize_with = "first")]
    current_condition: CurrentCondition,

    #[serde(rename = "weather")]
    days: Vec<Day>,
}

impl Weather {
    pub fn code(&self) -> &str {
        &self.current_condition.weather_code
    }

    pub fn feels(&self) -> &str {
        &self.current_condition.feels_like_c
    }

    pub fn tooltip(&self) -> String {
        let mut tooltip = String::new();
        tooltip.push_str(&format!(
            "<b>{} {}°C</b>\n",
            &self.current_condition.description, &self.current_condition.feels_like_c
        ));
        tooltip.push_str(&format!(
            "Wind: {}km/h\n",
            &self.current_condition.wind_speed
        ));
        tooltip.push_str(&format!(
            "Humidity: {}%\n",
            &self.current_condition.humidity
        ));

        let day_title = ["Today", "Tomorrow"];
        for (i, day) in self.days.iter().take(2).enumerate() {
            tooltip.push_str(&format!("\n<b>{}, {}</b>\n", day_title[i], day.date));
            tooltip.push_str(&format!(
                "⬆️ {}°C ⬇️ {}°C\n",
                day.min_temp_c, day.max_temp_c
            ));
            tooltip.push_str(&format!(
                "🌅 {}\n🌇 {}\n",
                day.astronomy.sunrise, day.astronomy.sunset
            ));

            for change in &day.changes {
                let icon = icon_from_id(&change.code);
                let padded = format!("{:04}", change.time.parse::<u16>().unwrap_or(0));
                let time = format!(
                    "  {}:{}",
                    padded.chars().take(2).collect::<String>(),
                    padded.chars().skip(2).take(2).collect::<String>(),
                );
                tooltip.push_str(&format!("{} {} {}°C\n", time, icon, change.fells_like_c));
            }
        }

        tooltip
    }
}

pub fn weather_for_city(city: &str) -> Result<Weather> {
    let city = city.trim();
    get(format!("https://wttr.in/{}?format=j1", city))
        .call()?
        .body_mut()
        .read_json::<Weather>()
        .map_err(From::from)
}

// serde deserializer to convert array of size 1 into their type
fn first<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let vec = Vec::<T>::deserialize(deserializer)?;
    vec.into_iter()
        .next()
        .ok_or_else(|| serde::de::Error::custom("expected one item"))
}

// serde deserializer to convert array of size one with { value: "..." } into String
fn first_value<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper {
        value: String,
    }

    let vec = Vec::<Wrapper>::deserialize(deserializer)?;
    vec.into_iter()
        .next()
        .map(|w| w.value)
        .ok_or_else(|| serde::de::Error::custom("expected on value item"))
}

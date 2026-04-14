use serde::{Deserialize, Deserializer};
use serde_with::{DisplayFromStr, serde_as};

use crate::config::Unit;

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct CurrentCondition {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "weatherCode")]
    pub weather_code: u16,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "FeelsLikeC")]
    feels_like_c: u8,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "FeelsLikeF")]
    feels_like_f: u8,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "temp_C")]
    temp_c: u8,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "temp_F")]
    temp_f: u8,

    #[serde(rename = "weatherDesc", deserialize_with = "first_value")]
    pub description: String,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "windspeedKmph")]
    pub wind_speed: u16,

    #[serde_as(as = "DisplayFromStr")]
    pub humidity: u8,
}

impl CurrentCondition {
    pub fn feels_like(&self, unit: &Unit) -> String {
        match unit {
            Unit::Celsius => c_deg(self.feels_like_c),
            Unit::Fahrenheit => f_deg(self.feels_like_f),
        }
    }

    pub fn temp(&self, unit: &Unit) -> String {
        match unit {
            Unit::Celsius => c_deg(self.temp_c),
            Unit::Fahrenheit => f_deg(self.temp_f),
        }
    }
}

fn c_deg(temperature: u8) -> String {
    format!("{}°C", temperature)
}

fn f_deg(temperature: u8) -> String {
    format!("{}°F", temperature)
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Day {
    pub date: String,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "maxtempC")]
    pub max_temp_c: u8,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "mintempC")]
    pub min_temp_c: u8,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "maxtempF")]
    pub max_temp_f: u8,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "mintempF")]
    pub min_temp_f: u8,

    #[serde(deserialize_with = "first")]
    pub astronomy: Astronomy,

    #[serde(rename = "hourly")]
    pub changes: Vec<HourlyChange>,
}

#[derive(Debug, Deserialize)]
pub struct Astronomy {
    pub sunrise: String,
    pub sunset: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct HourlyChange {
    pub time: String,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "weatherCode")]
    pub code: u16,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "FeelsLikeC")]
    pub feels_like_c: u8,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "FeelsLikeF")]
    pub feels_like_f: u8,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    #[serde(deserialize_with = "first")]
    pub current_condition: CurrentCondition,

    #[serde(rename = "weather")]
    pub days: Vec<Day>,
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

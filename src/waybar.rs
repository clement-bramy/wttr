use serde::{Deserialize, Serialize};

use crate::{config::Unit, domain::Weather, icons::icon_from_id};

#[derive(Serialize, Deserialize)]
pub struct Waybar {
    text: String,
    tooltip: String,
}

impl Waybar {
    pub fn from(weather: Weather, unit: Unit) -> Waybar {
        let mut tooltip = String::new();
        tooltip.push_str(&format!(
            "<b>{} {}°C</b>\nWind: {}km/h\nHumidity: {}%\n",
            &weather.current_condition.description,
            &weather.current_condition.feels_like(&unit),
            &weather.current_condition.wind_speed,
            &weather.current_condition.humidity,
        ));

        let day_title = ["Today", "Tomorrow"];
        for (i, day) in weather.days.iter().take(2).enumerate() {
            tooltip.push_str(&format!(
                "\n<b>{}, {}</b>\n⬆️ {}°C ⬇️ {}°C\n🌅 {}\n🌇 {}\n",
                day_title[i],
                day.date,
                day.min_temp_c,
                day.max_temp_c,
                day.astronomy.sunrise,
                day.astronomy.sunset
            ));

            for change in &day.changes {
                let icon = icon_from_id(change.code);
                let padded = format!("{:04}", change.time.parse::<u16>().unwrap_or(0));
                let time = format!(
                    "  {}:{}",
                    padded.chars().take(2).collect::<String>(),
                    padded.chars().skip(2).take(2).collect::<String>(),
                );
                tooltip.push_str(&format!("{} {} {}°C\n", time, icon, change.feels_like_c));
            }
        }

        let text = format!(
            "{} {}",
            icon_from_id(weather.current_condition.weather_code),
            weather.current_condition.feels_like(&unit),
        );

        Waybar { text, tooltip }
    }
}

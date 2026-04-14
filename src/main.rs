use std::process::exit;

use wttr::{config::load_config_or_default, weather_to_waybar};

fn main() {
    match weather_to_waybar(load_config_or_default()) {
        Ok(waybar) => print!("{}", waybar),
        Err(err) => {
            eprintln!("Failed to retrieve weather: {}", err);
            exit(1);
        }
    }
}

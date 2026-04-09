# 🌤️ wttr

A tiny Rust CLI that generates a **Waybar-friendly JSON output** for the current weather. Perfect for adding live weather info to your Waybar setup with style! 😎

---

## Features

- Displays **current temperature** with an icon 🎨  
- Shows **feels-like temperature** 🌡️  
- Provides a **detailed tooltip** with:
  - Wind speed 🌬️
  - Humidity 💧
  - Forecast for **today and tomorrow** 📅
  - Sunrise and sunset times 🌅🌇
  - Hourly forecast with icons ⏰  

- Tiny, **fast**, and **easy to integrate** with Waybar  
- Uses **serde** + **ureq** to fetch data from [wttr.in](https://wttr.in)

---

## Example output

```json
{
  "text": "⛅ 25°C",
  "tooltip": "<b>Partly cloudy 25°C</b>\nWind: 10km/h\nHumidity: 78%\n\n<b>Today, 2026-04-07</b>\n⬆️ 23°C ⬇️ 30°C\n🌅 06:10\n🌇 18:25\n  08:00 ⛅ 23°C\n..."
}

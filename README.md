# wttr

A minimal Waybar custom module that fetches weather data from [wttr.in](https://wttr.in) and formats it as a Waybar-compatible JSON payload.

## What it does

On each invocation, `wttr` calls the `wttr.in` JSON API, deserializes the response into typed Rust structs, and prints a single JSON object to stdout:

- **`text`** — a one-liner shown in the Waybar bar (weather icon + feels-like temperature)
- **`tooltip`** — a multi-line Pango-formatted popup with current conditions, wind, humidity, and a two-day forecast broken down by hour

The binary is designed to be called by Waybar's `exec` on a polling interval, not kept running.

![Waybar Preview](./docs/screenshot.webp)

## Requirements

- A working [Waybar](https://github.com/Alexays/Waybar) setup (Hyprland or any Wayland compositor)
- Network access to `wttr.in`

## Installation

```bash
cargo build --release
cp target/release/wttr ~/.local/bin/
```

The release profile is configured for minimal binary size (`opt-level = "z"`, LTO, single codegen unit, stripped symbols).

## Configuration

`wttr` looks for a TOML config file at the XDG config path:

```
~/.config/wttr/config.toml
```

If the file is absent or unparseable, defaults are used silently.

### Options

| Key    | Type                        | Default   | Description                        |
|--------|-----------------------------|-----------|------------------------------------|
| `city` | string (optional)           | auto-detect (wttr.in uses your IP) | City to fetch weather for |
| `unit` | `"celsius"` \| `"fahrenheit"` | `"celsius"` | Temperature unit                 |

### Example

```toml
city = "Paris"
unit = "celsius"
```

## Waybar integration

Add a custom module to your Waybar config:

```json
"custom/weather": {
    "exec": "wttr",
    "return-type": "json",
    "interval": 900,
    "format": "{}",
    "tooltip": true
}
```

Then add `"custom/weather"` to your bar's `modules-left`, `modules-center`, or `modules-right`.

## Crates used

| Crate        | Purpose                                              |
|--------------|------------------------------------------------------|
| `ureq`       | Blocking HTTP client with a 2-second global timeout  |
| `serde` / `serde_json` | JSON deserialization and serialization     |
| `serde_with` | `DisplayFromStr` adapter for number-as-string fields |
| `phf`        | Compile-time map from weather codes to emoji         |
| `toml`       | Config file parsing                                  |
| `directories`| XDG-compliant config path resolution                 |
| `anyhow`     | Error propagation                                    |

## License

MIT

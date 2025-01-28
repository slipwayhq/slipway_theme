use serde::{Deserialize, Serialize};

const NONE_STR: &str = "{}";
const COLOR_DARK_STR: &str = include_str!("../themes/color_dark.json");
const COLOR_LIGHT_STR: &str = include_str!("../themes/color_light.json");

wit_bindgen::generate!({
    world: "slipway",
});

struct Component;

export!(Component);

impl Guest for Component {
    fn run(input: String) -> Result<String, ComponentError> {
        let input: Input = serde_json::from_str(&input).expect("should parse JSON from stdin");

        // Get the input host config. If the input doesn't have a host config, use the default host config.
        // We always start off with a deserialized HostConfig struct so that all the defaults are materialized.
        // This makes manipulating the output of the theme component easier if the user wants to modify the
        // resulting host config.
        let input_host_config = input
            .host_config
            .unwrap_or_else(adaptive_cards_host_config::default);

        // Get the theme host config.
        let theme_str = match input.name {
            ThemeName::None => NONE_STR,
            ThemeName::ColorDark => COLOR_DARK_STR,
            ThemeName::ColorLight => COLOR_LIGHT_STR,
        };

        let theme_host_config_json: serde_json::Value =
            serde_json::from_str(theme_str).expect("should parse theme JSON");

        // Merge the theme host config into the input host config.
        let final_host_config_json =
            serde_json::to_value(input_host_config).expect("should serialize host config to JSON");

        let serde_json::Value::Object(theme_host_config_json) = theme_host_config_json else {
            panic!("theme host config should be an object");
        };

        let serde_json::Value::Object(mut final_host_config_json) = final_host_config_json else {
            panic!("host config should be an object");
        };

        for (key, value) in theme_host_config_json {
            final_host_config_json.insert(key, value);
        }

        // We don't serialize the final host config back to a HostConfig, and instead rely
        // on the component's output schema to ensure we're outputting a valid host config.
        let output = Output {
            host_config: serde_json::Value::Object(final_host_config_json),
        };

        Ok(serde_json::to_string(&output).expect("should serialize output to JSON"))
    }
}

#[derive(Deserialize)]
struct Input {
    name: ThemeName,

    /// The host config to modify with the theme.
    #[serde(rename = "hostConfig")]
    host_config: Option<adaptive_cards_host_config::HostConfig>,
}

#[derive(Deserialize)]
enum ThemeName {
    #[serde(alias = "none")]
    None,

    #[serde(alias = "color_dark")]
    ColorDark,

    #[serde(alias = "color_light")]
    ColorLight,
}

#[derive(Serialize)]
struct Output {
    #[serde(rename = "hostConfig")]
    host_config: serde_json::Value,
}

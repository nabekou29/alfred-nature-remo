use alfred;
use nature_remo;
use nature_remo::AirconModeValue;
use nature_remo::Appliance;
use serde_json::from_str;
use std::env;
use std::io;
use std::str;

fn main() {
    let app_json =
        env::var("appliance_json").unwrap_or(String::from("appliance_json is not defined"));
    let target = env::var("target").unwrap_or(String::from("target is not defined"));
    let app: Appliance = from_str(&app_json).unwrap();

    let aircon_setting = app.settings.unwrap();

    let items: Vec<alfred::Item> = match &target as &str {
        "button" => vec![
            alfred::ItemBuilder::new("ON")
                .variable("value", "")
                .into_item(),
            alfred::ItemBuilder::new("OFF")
                .variable("value", "power-off")
                .into_item(),
        ],
        "operation_mode" => vec![
            alfred::ItemBuilder::new("Auto")
                .variable("value", "auto")
                .into_item(),
            alfred::ItemBuilder::new("Cool")
                .variable("value", "cool")
                .into_item(),
            alfred::ItemBuilder::new("Warm")
                .variable("value", "warm")
                .into_item(),
            alfred::ItemBuilder::new("Dry")
                .variable("value", "dry")
                .into_item(),
            alfred::ItemBuilder::new("Blow")
                .variable("value", "blow")
                .into_item(),
        ],
        _ => {
            let modes = app.aircon.unwrap().range.modes;
            let mode_value: AirconModeValue = match &aircon_setting.mode as &str {
                "auto" => Ok(modes.auto),
                "cool" => Ok(modes.cool),
                "warm" => Ok(modes.warm),
                "dry" => Ok(modes.dry),
                "blow" => Ok(modes.blow),
                _ => Err(()),
            }
            .unwrap();

            let values = match &target as &str {
                "temperature" => Ok(mode_value.temp),
                "air_volume" => Ok(mode_value.vol),
                "air_direction" => Ok(mode_value.dir),
                _ => Err(()),
            }
            .unwrap();

            let items: Vec<alfred::Item> = values
                .into_iter()
                .map(|v| {
                    alfred::ItemBuilder::new(v.clone())
                        .variable("value", v)
                        .into_item()
                })
                .collect();

            items
        }
    };

    alfred::json::write_items(io::stdout(), &items).unwrap();
}

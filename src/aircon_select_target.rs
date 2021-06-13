use alfred;
use nature_remo;
use nature_remo::Appliance;
use serde_json::from_str;
use std::env;
use std::io;

fn main() {
    let app_json =
        env::var("appliance_json").unwrap_or(String::from("appliance_json is not defined"));
    let app: Appliance = from_str(&app_json).unwrap();

    let aircon_setting = app.settings.unwrap();

    // 選択肢作成
    let temp = alfred::ItemBuilder::new("Temperature")
        .variable("target", "temperature")
        .subtitle(format!(
            "Current: {}{}",
            aircon_setting.temp, aircon_setting.temp_unit
        ))
        .into_item();

    let vol = alfred::ItemBuilder::new("Air Volume")
        .variable("target", "air_volume")
        .subtitle(format!("Current: {}", aircon_setting.vol))
        .into_item();

    let dir = alfred::ItemBuilder::new("Air Direction")
        .variable("target", "air_direction")
        .subtitle(format!("Current: {}", aircon_setting.dir))
        .into_item();

    let mode = alfred::ItemBuilder::new("Mode")
        .variable("target", "operation_mode")
        .subtitle(format!("Current: {}", aircon_setting.mode))
        .into_item();

    let power = alfred::ItemBuilder::new("Power")
        .variable("target", "button")
        .subtitle(format!(
            "Current: {}",
            if aircon_setting.button == "" {
                "ON"
            } else {
                "OFF"
            }
        ))
        .into_item();

    alfred::json::write_items(io::stdout(), &[temp, vol, dir, mode, power]).unwrap();
}

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

    let light = app.light.unwrap();

    let items: Vec<alfred::Item> = light
        .buttons
        .into_iter()
        .map(|b| {
            alfred::ItemBuilder::new(b.name.clone())
                .variable("button_name", b.name)
                .into_item()
        })
        .collect();

    alfred::json::write_items(io::stdout(), &items).unwrap();
}

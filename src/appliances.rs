use alfred;
use nature_remo;
use nature_remo::AirconSettings;
use serde_json::json;
use std::env;
use std::io;

fn main() {
    let token = env::var("token").unwrap_or(String::from("token is not defined"));
    let client = nature_remo::Client::new(token);

    let appliances = client.get_appliances().unwrap();

    let items: Vec<alfred::Item> = appliances
        .into_iter()
        .map(|app| {
            let app_json = json!(app).to_string();

            let sub_title = app
                .settings
                .map(|s| aircon_setting_str(&s))
                .unwrap_or(String::from(""));

            alfred::ItemBuilder::new(app.nickname)
                .variable("id", app.id)
                .variable("type", app.type_)
                .variable("sub_title", sub_title.clone())
                .variable("appliance_json", app_json)
                .subtitle(sub_title)
                .into_item()
        })
        .collect();

    alfred::json::write_items(io::stdout(), &items).unwrap();
}

/// エアコンの設定を表示用の文字列に変換
fn aircon_setting_str(setting: &AirconSettings) -> String {
    format!(
        "モード:{}, 設定温度:{}, 風量:{}, 風向:{}",
        setting.mode, setting.temp, setting.vol, setting.dir
    )
}

use nature_remo;
use std::env;

fn main() {
    let token = env::var("token").unwrap_or(String::from("token is not defined"));

    let id = env::var("id").unwrap_or(String::from("id is not defined"));
    let target = env::var("target").unwrap_or(String::from("target is not defined"));
    let value = env::var("value").unwrap_or(String::from("value is not defined"));

    let client = nature_remo::Client::new(token);

    let mut params = nature_remo::RequestBody::new();
    params.insert(&target, &value);
    client.update_aircon_settings(&id, &params).unwrap();
}

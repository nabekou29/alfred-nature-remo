use nature_remo;
use std::env;

fn main() {
    let token = env::var("token").unwrap_or(String::from("token is not defined"));

    let id = env::var("id").unwrap_or(String::from("id is not defined"));
    let button_name = env::var("button_name").unwrap_or(String::from("button_name is not defined"));

    print!("{}, {}", id, button_name);
    let client = nature_remo::Client::new(token);
    client.update_light_state(&id, &button_name).unwrap();
}

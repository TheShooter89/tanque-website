use std::fmt::Display;

mod settings;
use settings::Settings;

mod utils;
use utils::*;

#[tokio::main]
async fn main() {
    let message = AppSplashMessage::from_title("TANQUE WEBSITE - WELCOME");

    println!("{}", message.to_string());
    // std::env::set_var("RUST_ENV", "default");

    let settings = Settings::load().unwrap();

    println!("SETTINGS LOADED:\n{:?}", settings)
}

use std::fmt::Display;

mod settings;
use settings::Settings;

mod utils;
use utils::*;

#[tokio::main]
async fn main() {
    let message = AppSplashMessage::from_title("TANQUE WEBSITE - WELCOME");

    println!("{}", message.to_string());

    let settings = Settings::load().unwrap();

    println!("SETTINGS LOADED:\n{:?}", settings)
}

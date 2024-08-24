#[macro_use]
extern crate lazy_static;

mod settings;
use settings::SETTINGS;

mod utils;
use utils::*;

#[tokio::main]
async fn main() {
    let message = AppSplashMessage::from_title("TANQUE WEBSITE - WELCOME");

    println!("{}", message.to_string());

    println!("SETTINGS LOADED:\n{:?}", *SETTINGS)
}

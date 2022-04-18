use clap::Parser;
use dotenv::dotenv;
use url_wrapper::config::Config;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
}

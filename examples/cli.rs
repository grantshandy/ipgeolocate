use locator::Locator;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    match Locator::get(args[1].to_string()) {
      Ok(ip) => println!("{} - {} ({})", ip.ip, ip.city, ip.region),
      Err(error) => println!("Error getting data: {}", error),
    };
}

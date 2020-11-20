use ipgeolocate::Locator;
use std::env;

// A simple CLI application for getting the city and country that an IP is located in.
fn main() {
    let args: Vec<_> = env::args().collect();

    match Locator::get(args[1].to_string().as_str()) {
      Ok(ip) => println!("{} - {} ({})", ip.ip, ip.city, ip.country),
      Err(error) => println!("Error getting data: {}", error),
    };
}

use ipgeolocate::Locator;
use std::env;

// A simple CLI application for getting the city and country that an IP is located in.
fn main() {
    let args: Vec<_> = env::args().collect();

    if args[1].to_string().as_str() == "ipwhois" {
        match Locator::ipwhois(args[2].to_string().as_str()) {
            Ok(ip) => println!("ipwhois: {} ({})", ip.ip, ip.country),
            Err(error) => println!("Error getting data: {}.", error),
        };
    } else if args[1].to_string().as_str() == "freegeoip" {
        match Locator::freegeoip(args[2].to_string().as_str()) {
            Ok(ip) => println!("freegeoip: {} ({})", ip.ip, ip.country),
            Err(error) => println!("Error getting data: {}.", error),
        };
    } else if args[1].to_string().as_str() == "ipapi" {
        match Locator::ipapi(args[2].to_string().as_str()) {
            Ok(ip) => println!("ipapi: {} ({})", ip.ip, ip.country),
            Err(error) => println!("Error getting data: {}.", error),
        };
    } else if args[1].to_string().as_str() == "ipapico" {
        match Locator::ipapico(args[2].to_string().as_str()) {
            Ok(ip) => println!("ipapico: {} ({})", ip.ip, ip.country),
            Err(error) => println!("Error getting data: {}.", error),
        };
    } else {
        eprintln!("cli: must use formatting 'cli <SERVICE> <IP>'");
    };
}

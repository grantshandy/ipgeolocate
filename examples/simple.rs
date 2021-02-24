use ipgeolocate::{Locator, Service};

// Prints the city where 1.1.1.1 is.
fn main() {
    let service = Service::IpApi;
    let ip = "1.1.1.1";

    match Locator::get(ip, service.clone()) {
        Ok(ip) => println!("{}: {} - {} ({})", service, ip.ip, ip.city, ip.country),
        Err(error) => println!("Error: {}", error),
    };
}

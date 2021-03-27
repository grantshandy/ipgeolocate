use ipgeolocate::{Locator, Service};
use std::net::Ipv4Addr;

// Prints the city where 1.1.1.1 is.
#[tokio::main]
async fn main() {
    let service = Service::IpApi;
    let ipv4 = Ipv4Addr::new(1, 1, 1, 1);

    match Locator::get_ipv4(ipv4, service).await {
        Ok(ip) => println!("{} - {} ({})", ip.ip, ip.city, ip.country),
        Err(error) => println!("Error: {}", error),
    };
}

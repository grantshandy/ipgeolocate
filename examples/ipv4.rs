use locator::Locator;
use std::net::Ipv4Addr;

// Prints the city where the Ipv4 1.1.1.1 is.
fn main() {
    let ip = Ipv4Addr::new(1, 1, 1, 1);

    match Locator::get_ipv4(ip) {
      Ok(ip) => println!("{} - {}", ip.ip, ip.city),
      Err(error) => println!("Error getting data: {}", error),
    };
}

use locator::Locator;
use std::net::Ipv6Addr;

// Prints the city where the Ipv6 ::1c9:0:0:afc8:0:1 is.
fn main() {
    let ip = Ipv6Addr::new(0, 0, 0x1c9, 0, 0, 0xafc8, 0, 0x1);

    match Locator::get_ipv6(ip) {
      Ok(ip) => println!("{} - {}", ip.ip, ip.city),
      Err(error) => println!("Error getting data: {}", error),
    };
}

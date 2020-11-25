use ipgeolocate::Locator;

// Prints the city where 1.1.1.1 is.
fn main() {
    match Locator::freegeoip("1.1.1.1") {
      Ok(ip) => println!("ipwhois: {} - {} ({})", ip.ip, ip.city, ip.country),
      Err(error) => println!("Error getting data: {}", error),
    };
}

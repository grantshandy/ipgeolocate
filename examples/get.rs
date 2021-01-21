use ipgeolocate::Locator;

// Prints the city and country where 1.1.1.1 is located.
fn main() {
    let service = "ipwhois";

    match Locator::get("1.1.1.1", service) {
      Ok(ip) => println!("{}: {} - {} ({})", service, ip.ip, ip.city, ip.country),
      Err(error) => println!("Error getting data: {}", error),
    };
}

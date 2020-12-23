use ipgeolocate::Locator;

// Prints the city where 1.1.1.1 is.
fn main() {
	let ip = "1.1.1.1";
	let service = "ipapico";

    match Locator::get(ip, service) {
      Ok(ip) => println!("{}: {} - {} ({})", service, ip.ip, ip.city, ip.country),
      Err(error) => println!("Error getting data: {}", error),
    };
}

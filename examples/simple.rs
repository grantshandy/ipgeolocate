use locator::Locator;

fn main() {
    match Locator::get("1.1.1.1".to_string()) {
      Ok(ip) => println!("{} - {}", ip.ip, ip.city),
      Err(error) => println!("Error getting data: {}", error),
    };
}

use ipgeolocate::{Locator, Service};

// Prints the city where 1.1.1.1 is.
#[tokio::main]
async fn main() {
    let service = Service::IpApiCo;
    let ip = "1.1.1.1";

    match Locator::get(ip, service).await {
        Ok(ip) => println!("{} - {} ({})", ip.ip, ip.city, ip.country),
        Err(error) => println!("Error: {}", error),
    };
}

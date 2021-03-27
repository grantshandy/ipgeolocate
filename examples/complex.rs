use ipgeolocate::{Locator, Service};

// Prints all the possible data points for 1.1.1.1.
#[tokio::main]
async fn main() {
    match Locator::get("1.1.1.1", Service::IpApi).await {
      Ok(ip) => println!("ipapi: {}\nLatitude: {}\nLongitude: {}\nCity: {}\nRegion: {}\nCountry: {}\nTimezone: {}", ip.ip, ip.latitude, ip.longitude, ip.city, ip.region, ip.country, ip.timezone),
      Err(error) => println!("Error getting data: {}", error),
    };
}

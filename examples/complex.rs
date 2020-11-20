use ipgeolocate::Locator;

// Prints all the possible data points for 1.1.1.1.
fn main() {
    match Locator::get("1.1.1.1") {
      Ok(ip) => println!("IP: {}\nLatitude: {}\nLongitude: {}\nCity: {}\nRegion: {}\nCountry Code: {}\nCountry: {}\nTimezone (GMT): {}\nTimezone: {}\nISP: {}\nIP type: {}", ip.ip, ip.latitude, ip.longitude, ip.city, ip.region, ip.country_code, ip.country, ip.timezone_gmt, ip.timezone, ip.isp, ip.iptype),
      Err(error) => println!("Error getting data: {}", error),
    };
}

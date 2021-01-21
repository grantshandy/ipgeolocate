# ipgeolocate
Get IP address geolocation information freely.

```
ipgeolocate = "0.3.0"
```
Add to `Cargo.toml`.

## Example
Using locator is really quite easy:
```
use ipgeolocate::Locator;

// Prints the city and country where 1.1.1.1 is located.
fn main() {
    let service = "ipwhois";

    match Locator::get("1.1.1.1", service) {
      Ok(ip) => println!("{}: {} - {} ({})", service, ip.ip, ip.city, ip.country),
      Err(error) => println!("Error getting data: {}", error),
    };
}
```

This and more examples are found in the examples directory.

## Query Limits
Each service included in this library has a weekly, hourly, or monthly limit.
Some have more free queries, but are less reliable.

Here are the query limits:

| Service       | Limit                     |
| ---------     | ------------------------- |
| ipwhois.app   | 10,000/month              |
| freegeoip.app | 15,000/hour               |
| ip-api.com    | 45/minute                 |
| ipapi.co      | 1,000/day (30,000/month)  |

You can use each of these just by running the function of the same name.

freegeoip.app is not recommended because it has issues reliably getting the correct latitude and longitude for IP addresses.

## Fields
The API can get these fields about IP addresses.

- ip
- latitude
- longitude
- city
- region
- country
- timezone

## Credits
Grant Handy <grantshandy@gmail.com>

APIs included:
- https://freegeoip.app/
- https://ipwhois.app/
- https://ip-api.com/
- https://ipapi.co/

Written with love, in Rust.

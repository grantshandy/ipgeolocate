# ipgeolocate
Get IP address geolocation information freely.

```
ipgeolocate = "0.3.5"
```
Add to `Cargo.toml`.

## Example
Because `ipgeolocate` is an async library, you need an async runtime like [`tokio`](https://crates.io/crates/tokio) or [`async-std`](https://crates.io/crates/async-std) to run.

Using `ipgeolocate` is really quite easy:
```
use ipgeolocate::{Locator, Service};

// Prints the city where 1.1.1.1 is.
#[tokio::main]
async fn main() {
    let service = Service::IpApi;
    let ip = "1.1.1.1";

    match Locator::get(ip, service).await {
        Ok(ip) => println!("{} - {} ({})", ip.ip, ip.city, ip.country),
        Err(error) => println!("Error: {}", error),
    };
}
```

This and more examples are found in the examples directory.

## Query Limits
Each service included in this library has a weekly, hourly, or monthly limit.
Some have more free queries, but are less reliable.

Here are the query limits:

| Service                                   | Limit                     |
| ---------                                 | ------------------------- |
| [ipwhois.app](https://freegeoip.app/)     | 10,000/month              |
| [freegeoip.app](https://ipwhois.app/)     | 15,000/hour               |
| [ip-api.com](https://ip-api.com/)         | 45/minute                 |
| [ipapi.co](https://ipapi.co/)             | 1,000/day (30,000/month)  |

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

# ipgeolocate
Get IP address geolocation information freely.

```
ipgeolocate = "0.2.8"
```
Add to `Cargo.toml`.

## Example
Using locator is really quite easy:
```
use locator::Locator;

fn main() {
    match Locator::ipapi("1.1.1.1") {
      Ok(ip) => println!("{} - {}", ip.ip, ip.city),
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

# ipgeolocate
Get IP address geolocation information freely.

```
ipgeolocate = "0.2.5"
```
Add to `Cargo.toml`.

## Example
Using locator is really quite easy:
```
use locator::Locator;

fn main() {
    match Locator::ipwhois("1.1.1.1") {
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

| Service   | Limit         |
| --------- | ------------- |
| ipwhois   | 10,000/month  |
| freegeoip | 15,000/hour   |

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

Data provided by https://freegeoip.app/

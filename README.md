# locator
Get IP address geolocation information freely.
```
locator = 0.2.1
```

## Example
Using locator is really quite easy:
```
use locator::Locator;

fn main() {
    match Locator::get("1.1.1.1") {
      Ok(ip) => println!("{} - {} ({})", ip.ip, ip.city, ip.country),
      Err(error) => println!("Error getting data: {}", error),
    };
}
```

This and more examples are found in the examples directory.

## Fields
The API can get these fields about IP addresses.

- ip
- latitude
- longitude
- city
- region
- country_code
- country
- timezone_gmt
- timezone
- isp
- iptype

## Credits
Grant Handy <grantshandy@gmail.com>

Data provided by [ipwhois](https://ipwhois.io/)

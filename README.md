# locator
Get IP address geolocation information freely.

Add to `Cargo.toml`.
```
locator = 0.2.2
```

## Example
Using locator is really quite easy:
```
use locator::Locator;
use std::net::Ipv4Addr;

fn main() {
    let ip = Ipv4Addr::new(1, 1, 1, 1);

    match Locator::get_ipv4(ip) {
      Ok(ip) => println!("{} - {}", ip.ip, ip.city),
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

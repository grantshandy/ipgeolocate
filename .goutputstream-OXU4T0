# locator
Get IP address geolocation information freely.
```
locator = 0.1.1
```

## Example
Using locator is really quite easy:
```
use locator::Locator;

fn main() {
    match Locator::get("1.1.1.1".to_string()) {
      Ok(ip) => println!("{} - {}", ip.ip, ip.city),
      Err(error) => println!("Error getting data: {}", error),
    };
}
```

Output:
```
1.1.1.1 - Los Angeles
```

## Fields

Currently there are only
- ip
- city
- latitude
- longitude

But more are planned in the future.

## Credits
Grant Handy <grantshandy@gmail.com>

Data provided by [ipwhois](https://ipwhois.io/)

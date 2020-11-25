#![feature(ip)]

use serde_json::Value;
use ureq::get;

pub struct Locator {
    pub ip: String,
    pub latitude: String,
    pub longitude: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub timezone: String,
}

impl Locator {
    pub fn freegeoip(ip: &str) -> std::result::Result<Self, String> {
        let url = format!("https://freegeoip.app/json/{}", ip);

        let response = get(&url).call();

        if !response.ok() {
            return Err(format!("Couldn't connect to freegeoip.app"));
        };

        // Turn the data into a string.
        let data = match response.into_string() {
            Ok(data) => data,
            Err(error) => {
                return Err(format!("Couldn't transform to string: {}", error));
            }
        };

        // Turn the data into parsed_json
        let parsed_json: Value = match serde_json::from_str(&data) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(format!("Couldn't parse json: {}", error));
            }
        };

        // Get latitude from parsed_json
        let latitude = match &parsed_json["latitude"] {
            Value::Number(latitude) => latitude,
            _ => {
                return Err("Unable to find latitude in parsed JSON".to_string())
            }
        };

        // Get longitude from parsed_json
        let longitude = match &parsed_json["longitude"] {
            Value::Number(longitude) => longitude,
            _ => {
                return Err("Unable to find longitude in parsed JSON".to_string());
            }
        };

        let city = match &parsed_json["city"] {
            Value::String(city_str) => city_str,
            _ => {
                return Err("Unable to find city in parsed JSON".to_string());
            }
        };

        let region = match &parsed_json["region_name"] {
            Value::String(region_str) => region_str,
            _ => {
                return Err("Unable to find region in parsed JSON".to_string());
            }
        };

        let country = match &parsed_json["country_name"] {
            Value::String(country_str) => country_str,
            _ => {
                return Err("Unable to find country in parsed JSON".to_string());
            }
        };

        let timezone = match &parsed_json["time_zone"] {
            Value::String(timezone_str) => timezone_str,
            _ => {
                return Err("Unable to find timezone in parsed JSON".to_string());
            }
        };

        let ip = ip.to_string();
        let latitude = latitude.to_string();
        let longitude = longitude.to_string();
        let city = city.to_string();
        let region = region.to_string();
        let country = country.to_string();
        let timezone = timezone.to_string();

        let result = Locator {
            ip,
            latitude,
            longitude,
            city,
            region,
            country,
            timezone,
        };

        Ok(result)
    }

    pub fn ipwhois(ip: &str) -> std::result::Result<Self, String> {
        let url = format!("http://ipwhois.app/json/{}", ip);

        let response = get(&url).call();

        if !response.ok() {
            return Err(format!("Couldn't connect to ipwhois.app"));
        };

        // Turn the data into a string.
        let data = match response.into_string() {
            Ok(data) => data,
            Err(error) => {
                return Err(format!("Error transforming to string: {}", error));
            }
        };

        // Turn the data into parsed_json
        let parsed_json: Value = match serde_json::from_str(&data) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(format!("Error parsing json: {}", error));
            }
        };

        let success = match &parsed_json["success"] {
            Value::Bool(latitude_str) => latitude_str,
            _ => {
                return Err("Cannot find success in JSON".to_string());
            }
        };

        if !success {
            return Err("You've hit the monthly limit".to_string());
        }

        // Get latitude from parsed_json
        let latitude_str = match &parsed_json["latitude"] {
            Value::String(latitude_str) => latitude_str,
            _ => {
                return Err("Unable to find latitude in parsed JSON".to_string());
            }
        };

        // Get longitude from parsed_json
        let longitude_str = match &parsed_json["longitude"] {
            Value::String(longitude_str) => longitude_str,
            _ => {
                return Err("Unable to find longitude in parsed JSON".to_string());
            }
        };

        let city_str = match &parsed_json["city"] {
            Value::String(city_str) => city_str,
            _ => {
                return Err("Unable to find city in parsed JSON".to_string());
            }
        };

        let region_str = match &parsed_json["region"] {
            Value::String(region_str) => region_str,
            _ => {
                return Err("Unable to find region in parsed JSON".to_string());
            }
        };

        let country_str = match &parsed_json["country"] {
            Value::String(country_str) => country_str,
            _ => {
                return Err("Unable to find country in parsed JSON".to_string());
            }
        };

        let timezone_str = match &parsed_json["timezone"] {
            Value::String(timezone_str) => timezone_str,
            _ => {
                return Err("Unable to find timezone in parsed JSON".to_string());
            }
        };

        let ip = ip.to_string();
        let latitude = latitude_str.to_string();
        let longitude = longitude_str.to_string();
        let city = city_str.to_string();
        let region = region_str.to_string();
        let country = country_str.to_string();
        let timezone = timezone_str.to_string();

        let result = Locator {
            ip,
            latitude,
            longitude,
            city,
            region,
            country,
            timezone,
        };

        Ok(result)
    }

    pub fn ipapi(ip: &str) -> std::result::Result<Self, String> {
        let url = format!("http://ip-api.com/json/{}", ip);

        let response = get(&url).call();

        if !response.ok() {
            return Err(format!("Couldn't connect to ip-api.com"));
        };

        // Turn the data into a string.
        let data = match response.into_string() {
            Ok(data) => data,
            Err(error) => {
                return Err(format!("Couldn't transform to string: {}", error));
            }
        };

        // Turn the data into parsed_json
        let parsed_json: Value = match serde_json::from_str(&data) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(format!("Couldn't parse json: {}", error));
            }
        };

        // Get latitude from parsed_json
        let latitude = match &parsed_json["lat"] {
            Value::Number(latitude) => latitude,
            _ => {
                return Err("Unable to find latitude in parsed JSON".to_string())
            }
        };

        // Get longitude from parsed_json
        let longitude = match &parsed_json["lon"] {
            Value::Number(longitude) => longitude,
            _ => {
                return Err("Unable to find longitude in parsed JSON".to_string());
            }
        };

        let city = match &parsed_json["city"] {
            Value::String(city_str) => city_str,
            _ => {
                return Err("Unable to find city in parsed JSON".to_string());
            }
        };

        let region = match &parsed_json["regionName"] {
            Value::String(region_str) => region_str,
            _ => {
                return Err("Unable to find region in parsed JSON".to_string());
            }
        };

        let country = match &parsed_json["country"] {
            Value::String(country_str) => country_str,
            _ => {
                return Err("Unable to find country in parsed JSON".to_string());
            }
        };

        let timezone = match &parsed_json["timezone"] {
            Value::String(timezone_str) => timezone_str,
            _ => {
                return Err("Unable to find timezone in parsed JSON".to_string());
            }
        };

        let ip = ip.to_string();
        let latitude = latitude.to_string();
        let longitude = longitude.to_string();
        let city = city.to_string();
        let region = region.to_string();
        let country = country.to_string();
        let timezone = timezone.to_string();

        let result = Locator {
            ip,
            latitude,
            longitude,
            city,
            region,
            country,
            timezone,
        };

        Ok(result)
    }

    pub fn ipapico(ip: &str) -> std::result::Result<Self, String> {
        let url = format!("http://ipapi.co/{}/json/", ip);

        let response = get(&url).call();

        if !response.ok() {
            return Err(format!("Couldn't connect to ipapi.co"));
        };

        // Turn the data into a string.
        let data = match response.into_string() {
            Ok(data) => data,
            Err(error) => {
                return Err(format!("Couldn't transform to string: {}", error));
            }
        };

        // Turn the data into parsed_json
        let parsed_json: Value = match serde_json::from_str(&data) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(format!("Couldn't parse json: {}", error));
            }
        };

        // Get latitude from parsed_json
        let latitude = match &parsed_json["latitude"] {
            Value::Number(latitude) => latitude,
            _ => {
                return Err("Unable to find latitude in parsed JSON".to_string())
            }
        };

        // Get longitude from parsed_json
        let longitude = match &parsed_json["longitude"] {
            Value::Number(longitude) => longitude,
            _ => {
                return Err("Unable to find longitude in parsed JSON".to_string());
            }
        };

        let city = match &parsed_json["city"] {
            Value::String(city_str) => city_str,
            _ => {
                return Err("Unable to find city in parsed JSON".to_string());
            }
        };

        let region = match &parsed_json["region"] {
            Value::String(region_str) => region_str,
            _ => {
                return Err("Unable to find region in parsed JSON".to_string());
            }
        };

        let country = match &parsed_json["country_name"] {
            Value::String(country_str) => country_str,
            _ => {
                return Err("Unable to find country in parsed JSON".to_string());
            }
        };

        let timezone = match &parsed_json["timezone"] {
            Value::String(timezone_str) => timezone_str,
            _ => {
                return Err("Unable to find timezone in parsed JSON".to_string());
            }
        };

        let ip = ip.to_string();
        let latitude = latitude.to_string();
        let longitude = longitude.to_string();
        let city = city.to_string();
        let region = region.to_string();
        let country = country.to_string();
        let timezone = timezone.to_string();

        let result = Locator {
            ip,
            latitude,
            longitude,
            city,
            region,
            country,
            timezone,
        };

        Ok(result)
    }
}

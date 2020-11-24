#![feature(ip)]

use serde_json::Value;
use ureq::get;
use std::net::{Ipv4Addr, Ipv6Addr};

pub struct Locator {
    pub ip: String,
    pub latitude: f64,
    pub longitude: f64,
    pub city: String,
    pub region: String,
    pub country: String,
    pub timezone: String,
}

impl Locator {
    pub fn get_ipv4(ip: Ipv4Addr) -> std::result::Result<Self, String> {
        if ip.is_private() {
            return Err(format!("IP can't be private"));
        } else {
            let locate = Locator::get(ip.to_string().as_str());
            locate
        }
    }
    /// Gets the Locator information for [`Ipv4Addr`].

    pub fn get_ipv6(ip: Ipv6Addr) -> std::result::Result<Self, String> {
        if !ip.is_global() {
            return Err(format!("IP can't be private"));
        } else {
            let locate = Locator::get(ip.to_string().as_str());
            locate
        }
    }
    /// Gets the Locator information for [`Ipv6Addr`].

    pub fn get(ip: &str) -> std::result::Result<Self, String> {
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
                return Err("Unable to find latitude in parsed JSON".to_string());
            }
        };

        let latitude = match latitude.as_f64() {
            Some(latitude) => latitude,
            _ => {
                return Err("latitude not f64".to_string());
            }
        };

        // Get longitude from parsed_json
        let longitude = match &parsed_json["longitude"] {
            Value::Number(longitude) => longitude,
            _ => {
                return Err("Unable to find longitude in parsed JSON".to_string());
            }
        };

        let longitude = match longitude.as_f64() {
            Some(longitude) => longitude,
            _ => {
                return Err("longitude not f64".to_string());
            }
        };

        let city_str = match &parsed_json["city"] {
            Value::String(city_str) => city_str,
            _ => {
                return Err("Unable to find city in parsed JSON".to_string());
            }
        };

        let region_str = match &parsed_json["region_name"] {
            Value::String(region_str) => region_str,
            _ => {
                return Err("Unable to find region in parsed JSON".to_string());
            }
        };

        let country_str = match &parsed_json["country_name"] {
            Value::String(country_str) => country_str,
            _ => {
                return Err("Unable to find country in parsed JSON".to_string());
            }
        };

        let timezone_str = match &parsed_json["time_zone"] {
            Value::String(timezone_str) => timezone_str,
            _ => {
                return Err("Unable to find timezone in parsed JSON".to_string());
            }
        };

        let ip = ip.to_string();
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
}

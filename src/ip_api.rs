#![feature(ip)]

use serde_json::Value;
use ureq::get;
use std::net::{Ipv4Addr, Ipv6Addr};

pub struct Query {
    pub ip: String,
    pub latitude: String,
    pub longitude: String,
    pub city: String,
}

impl Query {
    pub fn get_ipv4(ip: Ipv4Addr) -> std::result::Result<Self, String> {
        if ip.is_private() {
            return Err(format!("IP can't be private"));
        } else {
            let locate = Query::get(ip.to_string().as_str());
            locate
        }
    }
    /// Gets the Locator information for [`Ipv4Addr`].

    pub fn get_ipv6(ip: Ipv6Addr) -> std::result::Result<Self, String> {
        if !ip.is_global() {
            return Err(format!("IP can't be private"));
        } else {
            let locate = Query::get(ip.to_string().as_str());
            locate
        }
    }
    /// Gets the Locator information for [`Ipv6Addr`].

    pub fn get(ip: &str) -> std::result::Result<Self, String> {
        let url = format!("http://ip-api.com/json/{}", ip);

        let response = get(&url).call();

        if !response.ok() {
            return Err(format!("Couldn't connect to ip-api.com. You were probably rate limited."));
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
        let latitude_str = match &parsed_json["lat"] {
            Value::String(latitude_str) => latitude_str,
            _ => {
                return Err("Unable to find latitude in parsed JSON".to_string());
            }
        };

        // Get longitude from parsed_json
        let longitude_str = match &parsed_json["lon"] {
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

        let ip = ip.to_string();
        let latitude = latitude_str.to_string();
        let longitude = longitude_str.to_string();
        let city = city_str.to_string();

        let result = Query {
            ip,
            latitude,
            longitude,
            city,
        };

        Ok(result)
    }
}

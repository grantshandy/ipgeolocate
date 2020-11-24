use serde_json::Value;
use ureq::get;
use std::net::{Ipv4Addr, Ipv6Addr};

pub struct Locator {
    pub ip: String,
    pub latitude: String,
    pub longitude: String,
    pub city: String,
    pub region: String,
    pub country_code: String,
    pub country: String,
    pub timezone_gmt: String,
    pub timezone: String,
    pub iptype: String,
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
        let url = format!("http://ipwhois.app/json/{}", ip);

        let response = get(&url).call();

        if !response.ok() {
            return Err(format!("Couldn't connect to ipwhois.app"));
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

        let country_code_str = match &parsed_json["country_code"] {
            Value::String(country_code_str) => country_code_str,
            _ => {
                return Err("Unable to find country_code in parsed JSON".to_string());
            }
        };

        let country_str = match &parsed_json["country"] {
            Value::String(country_str) => country_str,
            _ => {
                return Err("Unable to find country in parsed JSON".to_string());
            }
        };

        let timezone_gmt_str = match &parsed_json["timezone_gmt"] {
            Value::String(timezone_gmt_str) => timezone_gmt_str,
            _ => {
                return Err("Unable to find timezone_gmt in parsed JSON".to_string());
            }
        };

        let timezone_str = match &parsed_json["timezone"] {
            Value::String(timezone_str) => timezone_str,
            _ => {
                return Err("Unable to find timezone in parsed JSON".to_string());
            }
        };

        let iptype_str = match &parsed_json["type"] {
            Value::String(type_str) => type_str,
            _ => {
                return Err("Unable to find type in parsed JSON".to_string());
            }
        };

        let ip = ip.to_string();
        let latitude = latitude_str.to_string();
        let longitude = longitude_str.to_string();
        let city = city_str.to_string();
        let region = region_str.to_string();
        let country_code = country_code_str.to_string();
        let country = country_str.to_string();
        let timezone_gmt = timezone_gmt_str.to_string();
        let timezone = timezone_str.to_string();
        let iptype = iptype_str.to_string();

        let result = Locator {
            ip,
            latitude,
            longitude,
            city,
            region,
            country_code,
            country,
            timezone_gmt,
            timezone,
            iptype,
        };

        Ok(result)
    }
}

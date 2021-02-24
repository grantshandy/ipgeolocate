//! # ipgeolocate
//! Get IP address geolocation information freely.

//! ```
//! ipgeolocate = "0.3.1"
//! ```
//! Add to `Cargo.toml`.

//! ## Example
//! Using locator is really quite easy:
//! ```
//! use ipgeolocate::{Locator, Service};

//! Prints the city where 1.1.1.1 is.
//! fn main() {
//!     match Locator::get("1.1.1.1", Service::IpApi) {
//!       Ok(ip) => println!("ipapi: {} - {} ({})", ip.ip, ip.city, ip.country),
//!       Err(error) => println!("Error getting data: {}", error),
//!     };
//! }
//! ```

//! This and more examples are found in the examples directory.

//! ## Query Limits
//! Each service included in this library has a weekly, hourly, or monthly limit.
//! Some have more free queries, but are less reliable.

//! Here are the query limits:

//! | Service       | Limit                     |
//! | ---------     | ------------------------- |
//! | ipwhois.app   | 10,000/month              |
//! | freegeoip.app | 15,000/hour               |
//! | ip-api.com    | 45/minute                 |
//! | ipapi.co      | 1,000/day (30,000/month)  |

//! You can use each of these just by running the function of the same name.

//! freegeoip.app is not recommended because it has issues reliably getting the correct latitude and longitude for IP addresses.

//! ## Fields
//! The API can get these fields about IP addresses.

//! - ip
//! - latitude
//! - longitude
//! - city
//! - region
//! - country
//! - timezone

//! ## Credits
//! Grant Handy <grantshandy@gmail.com>

//! APIs included:
//! - https://freegeoip.app/
//! - https://ipwhois.app/
//! - https://ip-api.com/
//! - https://ipapi.co/

// Written with love, in Rust.

use serde_json::Value;
use std::fmt;
use std::net::IpAddr;
use ureq::get;

/// Services (apis) that can be used for accessing geolocation data.
#[derive(Debug, Clone)]
pub enum Service {
    IpWhois,
    IpApi,
    IpApiCo,
    FreeGeoIp,
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Service::IpWhois => {
                write!(f, "ipwhois")
            }
            Service::IpApi => {
                write!(f, "ipapi")
            }
            Service::IpApiCo => {
                write!(f, "ipapico")
            }
            Service::FreeGeoIp => {
                write!(f, "freegeoip")
            }
        }
    }
}

#[derive(Debug, Clone)]
/// A general ipgeolocate error for requests.
pub enum GeoError {
    HttpError(String),
    ParseError(String),
}

impl std::error::Error for GeoError {}

impl fmt::Display for GeoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GeoError::HttpError(error) => {
                write!(f, "HTTP Request Error: {}", error.to_string())
            }
            GeoError::ParseError(error) => {
                write!(f, "JSON Parsing Error: {}", error.to_string())
            }
        }
    }
}

/// This is the main struct for making requests to the APIs.
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
    /// Gets IP information from an `IpAddr`
    pub fn get_ipaddr(ip: IpAddr, service: Service) -> std::result::Result<Self, GeoError> {
        Locator::get(&ip.to_string(), service)
    }

    /// Gets IP information from just a string (not reccomended for most uses)
    pub fn get(ip: &str, service: Service) -> std::result::Result<Self, GeoError> {
        match service {
            Service::IpWhois => Locator::ipwhois(ip),
            Service::IpApi => Locator::ipapi(ip),
            Service::IpApiCo => Locator::ipapico(ip),
            Service::FreeGeoIp => Locator::freegeoip(ip),
        }
    }

    fn freegeoip(ip: &str) -> std::result::Result<Self, GeoError> {
        let url = format!("https://freegeoip.app/json/{}", ip);

        let response = get(&url).call();

        if !response.ok() {
            return Err(GeoError::HttpError(format!(
                "Couldn't connect to freegeoip.app"
            )));
        };

        // Turn the data into a string.
        let data = match response.into_string() {
            Ok(data) => data,
            Err(error) => {
                return Err(GeoError::ParseError(format!(
                    "Couldn't transform to string: {}",
                    error
                )));
            }
        };

        // Turn the data into parsed_json
        let parsed_json: Value = match serde_json::from_str(&data) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(GeoError::ParseError(format!(
                    "Couldn't parse json: {}",
                    error
                )));
            }
        };

        // Get latitude from parsed_json
        let latitude = match &parsed_json["latitude"] {
            Value::Number(latitude) => latitude,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find latitude in parsed JSON".to_string(),
                ))
            }
        };

        // Get longitude from parsed_json
        let longitude = match &parsed_json["longitude"] {
            Value::Number(longitude) => longitude,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find longitude in parsed JSON".to_string(),
                ));
            }
        };

        let city = match &parsed_json["city"] {
            Value::String(city_str) => city_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find city in parsed JSON".to_string(),
                ));
            }
        };

        let region = match &parsed_json["region_name"] {
            Value::String(region_str) => region_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find region in parsed JSON".to_string(),
                ));
            }
        };

        let country = match &parsed_json["country_name"] {
            Value::String(country_str) => country_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find country in parsed JSON".to_string(),
                ));
            }
        };

        let timezone = match &parsed_json["time_zone"] {
            Value::String(timezone_str) => timezone_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find timezone in parsed JSON".to_string(),
                ));
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

    fn ipwhois(ip: &str) -> std::result::Result<Self, GeoError> {
        let url = format!("http://ipwhois.app/json/{}", ip);

        let response = get(&url).call();

        if !response.ok() {
            return Err(GeoError::HttpError(format!(
                "Couldn't connect to ipwhois.app"
            )));
        };

        // Turn the data into a string.
        let data = match response.into_string() {
            Ok(data) => data,
            Err(error) => {
                return Err(GeoError::ParseError(format!(
                    "Error transforming to string: {}",
                    error
                )));
            }
        };

        // Turn the data into parsed_json
        let parsed_json: Value = match serde_json::from_str(&data) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(GeoError::ParseError(format!(
                    "Error parsing json: {}",
                    error
                )));
            }
        };

        let success = match &parsed_json["success"] {
            Value::Bool(latitude_str) => latitude_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Cannot find success in JSON".to_string(),
                ));
            }
        };

        if !success {
            return Err(GeoError::ParseError(
                "You've hit the monthly limit".to_string(),
            ));
        }

        // Get latitude from parsed_json
        let latitude_str = match &parsed_json["latitude"] {
            Value::String(latitude_str) => latitude_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find latitude in parsed JSON".to_string(),
                ));
            }
        };

        // Get longitude from parsed_json
        let longitude_str = match &parsed_json["longitude"] {
            Value::String(longitude_str) => longitude_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find longitude in parsed JSON".to_string(),
                ));
            }
        };

        let city_str = match &parsed_json["city"] {
            Value::String(city_str) => city_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find city in parsed JSON".to_string(),
                ));
            }
        };

        let region_str = match &parsed_json["region"] {
            Value::String(region_str) => region_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find region in parsed JSON".to_string(),
                ));
            }
        };

        let country_str = match &parsed_json["country"] {
            Value::String(country_str) => country_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find country in parsed JSON".to_string(),
                ));
            }
        };

        let timezone_str = match &parsed_json["timezone"] {
            Value::String(timezone_str) => timezone_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find timezone in parsed JSON".to_string(),
                ));
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

    fn ipapi(ip: &str) -> std::result::Result<Self, GeoError> {
        let url = format!("http://ip-api.com/json/{}", ip);

        let response = get(&url).call();

        if !response.ok() {
            return Err(GeoError::HttpError(format!(
                "Couldn't connect to ip-api.com"
            )));
        };

        // Turn the data into a string.
        let data = match response.into_string() {
            Ok(data) => data,
            Err(error) => {
                return Err(GeoError::ParseError(format!(
                    "Couldn't transform to string: {}",
                    error
                )));
            }
        };

        // Turn the data into parsed_json
        let parsed_json: Value = match serde_json::from_str(&data) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(GeoError::ParseError(format!(
                    "Couldn't parse json: {}",
                    error
                )));
            }
        };

        // Get latitude from parsed_json
        let latitude = match &parsed_json["lat"] {
            Value::Number(latitude) => latitude,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find latitude in parsed JSON".to_string(),
                ));
            }
        };

        // Get longitude from parsed_json
        let longitude = match &parsed_json["lon"] {
            Value::Number(longitude) => longitude,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find longitude in parsed JSON".to_string(),
                ));
            }
        };

        let city = match &parsed_json["city"] {
            Value::String(city_str) => city_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find city in parsed JSON".to_string(),
                ));
            }
        };

        let region = match &parsed_json["regionName"] {
            Value::String(region_str) => region_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find region in parsed JSON".to_string(),
                ));
            }
        };

        let country = match &parsed_json["country"] {
            Value::String(country_str) => country_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find country in parsed JSON".to_string(),
                ));
            }
        };

        let timezone = match &parsed_json["timezone"] {
            Value::String(timezone_str) => timezone_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find timezone in parsed JSON".to_string(),
                ));
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

    fn ipapico(ip: &str) -> std::result::Result<Self, GeoError> {
        let url = format!("http://ipapi.co/{}/json/", ip);

        let response = get(&url).call();

        if !response.ok() {
            return Err(GeoError::HttpError(format!("Couldn't connect to ipapi.co")));
        };

        // Turn the data into a string.
        let data = match response.into_string() {
            Ok(data) => data,
            Err(error) => {
                return Err(GeoError::ParseError(format!(
                    "Couldn't transform to string: {}",
                    error
                )));
            }
        };

        // Turn the data into parsed_json
        let parsed_json: Value = match serde_json::from_str(&data) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(GeoError::ParseError(format!(
                    "Couldn't parse json: {}",
                    error
                )));
            }
        };

        // Get latitude from parsed_json
        let latitude = match &parsed_json["latitude"] {
            Value::Number(latitude) => latitude,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find latitude in parsed JSON".to_string(),
                ))
            }
        };

        // Get longitude from parsed_json
        let longitude = match &parsed_json["longitude"] {
            Value::Number(longitude) => longitude,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find longitude in parsed JSON".to_string(),
                ));
            }
        };

        let city = match &parsed_json["city"] {
            Value::String(city_str) => city_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find city in parsed JSON".to_string(),
                ));
            }
        };

        let region = match &parsed_json["region"] {
            Value::String(region_str) => region_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find region in parsed JSON".to_string(),
                ));
            }
        };

        let country = match &parsed_json["country_name"] {
            Value::String(country_str) => country_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find country in parsed JSON".to_string(),
                ));
            }
        };

        let timezone = match &parsed_json["timezone"] {
            Value::String(timezone_str) => timezone_str,
            _ => {
                return Err(GeoError::ParseError(
                    "Unable to find timezone in parsed JSON".to_string(),
                ));
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

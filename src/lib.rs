//! # ipgeolocate
//! Get IP address geolocation information freely.
//!
//! ```
//! ipgeolocate = "0.3.5"
//! ```
//! Add to `Cargo.toml`.
//!
//! ## Example
//! Because `ipgeolocate` is an async library, you need an async runtime like [`tokio`](https://crates.io/crates/tokio) or [`async-std`](https://crates.io/crates/async-std) to run.
//!
//! Using `ipgeolocate` is really quite easy:
//! ```
//! use ipgeolocate::{Locator, Service};
//!
//! // Prints the city where 1.1.1.1 is.
//! #[tokio::main]
//! async fn main() {
//!     let service = Service::IpApi;
//!     let ip = "1.1.1.1";
//!
//!     match Locator::get(ip, service).await {
//!         Ok(ip) => println!("{} - {} ({})", ip.ip, ip.city, ip.country),
//!         Err(error) => println!("Error: {}", error),
//!     };
//! }
//! ```
//!
//! This and more examples are found in the examples directory.
//!
//! ## Query Limits
//! Each service included in this library has a weekly, hourly, or monthly limit.
//! Some have more free queries, but are less reliable.
//!
//! Here are the query limits:
//!
//! | Service                                   | Limit                     |
//! | ---------                                 | ------------------------- |
//! | [ipwhois.app](https://freegeoip.app/)     | 10,000/month              |
//! | [freegeoip.app](https://ipwhois.app/)     | 15,000/hour               |
//! | [ip-api.com](https://ip-api.com/)         | 45/minute                 |
//! | [ipapi.co](https://ipapi.co/)             | 1,000/day (30,000/month)  |
//!
//! You can use each of these just by running the function of the same name.
//!
//! freegeoip.app is not recommended because it has issues reliably getting the correct latitude and longitude for IP addresses.
//!
//! ## Fields
//! The API can get these fields about IP addresses.
//!
//! - [`ip`](crate::Locator::ip)
//! - [`latitude`](crate::Locator::latitude)
//! - [`longitude`](crate::Locator::longitude)
//! - [`city`](crate::Locator::city)
//! - [`region`](crate::Locator::region)
//! - [`country`](crate::Locator::country)
//! - [`timezone`](crate::Locator::timezone)
//!
//! ## Credits
//! Grant Handy <grantshandy@gmail.com>
//!
//! APIs included:
//! - [ipwhois.app](https://freegeoip.app/)
//! - [freegeoip.app](https://ipwhois.app/)
//! - [ip-api.com](https://ip-api.com/)
//! - [ipapi.co](https://ipapi.co/)
//!
//! Written with love, in Rust.
//!

use reqwest::get;
use serde_json::Value;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use tracing::debug;

/// Services (apis) that can be used for accessing geolocation data.
#[derive(Debug, Clone, Copy)]
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
                write!(f, "HTTP Request Error: {}", error)
            }
            GeoError::ParseError(error) => {
                write!(f, "JSON Parsing Error: {}", error)
            }
        }
    }
}

/// This is the main struct for making requests to the APIs.
#[derive(Debug, Clone)]
pub struct Locator {
    /// Returns the IP address.
    pub ip: String,
    /// Latitude of the IP address.
    pub latitude: String,
    /// Longitude of the IP address.
    pub longitude: String,
    /// City of the IP address.
    pub city: String,
    /// Region or state of the IP address.
    pub region: String,
    /// Country of the IP address.
    pub country: String,
    /// Timezone of the IP address.
    pub timezone: String,
    /// ISP of the IP address
    pub isp: String,
}

impl Locator {
    /// Gets IP information from an [`Ipv4Addr`]
    pub async fn get_ipv4(ip: Ipv4Addr, service: Service) -> std::result::Result<Self, GeoError> {
        Locator::get(&ip.to_string(), service).await
    }

    /// Gets IP information from an [`Ipv6Addr`]
    pub async fn get_ipv6(ip: Ipv6Addr, service: Service) -> std::result::Result<Self, GeoError> {
        Locator::get(&ip.to_string(), service).await
    }

    /// Gets IP information from an [`IpAddr`]
    pub async fn get_ipaddr(ip: IpAddr, service: Service) -> std::result::Result<Self, GeoError> {
        Locator::get(&ip.to_string(), service).await
    }

    /// [`IpAddr`]: std::net::IpAddr
    /// [`Ipv4Addr`]: std::net::Ipv4Addr
    /// [`Ipv6Addr`]: std::net::Ipv6Addr

    /// Gets IP information from just a string (not recommended for most uses)
    pub async fn get(ip: &str, service: Service) -> std::result::Result<Self, GeoError> {
        match service {
            Service::IpWhois => Locator::ipwhois(ip).await,
            Service::IpApi => Locator::ipapi(ip).await,
            Service::IpApiCo => Locator::ipapico(ip).await,
            Service::FreeGeoIp => Locator::freegeoip(ip).await,
        }
    }

    async fn freegeoip(ip: &str) -> std::result::Result<Self, GeoError> {
        let url = format!("https://freegeoip.app/json/{}", ip);

        let response = match get(&url).await.unwrap().text().await {
            Ok(response) => response,
            Err(_) => {
                return Err(GeoError::HttpError(
                    "Couldn't connect to freegeoip.app".to_string(),
                ))
            }
        };

        // Turn the data into parsed_json
        let parsed_json: Value = match serde_json::from_str(&response) {
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
        let isp = String::default();

        let result = Locator {
            ip,
            latitude,
            longitude,
            city,
            region,
            country,
            timezone,
            isp,
        };

        Ok(result)
    }

    async fn ipwhois(ip: &str) -> std::result::Result<Self, GeoError> {
        let url = format!("http://ipwhois.app/json/{}", ip);

        let response = match get(&url).await.unwrap().text().await {
            Ok(response) => response,
            Err(_) => {
                return Err(GeoError::HttpError(
                    "Couldn't connect to ipwhois.app".to_string(),
                ))
            }
        };

        // Turn the data into parsed_json
        let parsed_json: Value = match serde_json::from_str(&response) {
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
            isp: String::default(),
        };

        Ok(result)
    }

    async fn ipapi(ip: &str) -> std::result::Result<Self, GeoError> {
        let url = format!("http://ip-api.com/json/{}", ip);

        let response = match get(&url).await.unwrap().text().await {
            Ok(response) => response,
            Err(_) => {
                return Err(GeoError::HttpError(
                    "Couldn't connect to ip-api.com".to_string(),
                ))
            }
        };

        // Turn the data into parsed_json
        let parsed_json: Value = match serde_json::from_str(&response) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(GeoError::ParseError(format!(
                    "Couldn't parse json: {}",
                    error
                )));
            }
        };

        debug!("ipgeolocate return object looks like: {}", parsed_json);

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

        let isp = match &parsed_json["isp"] {
            Value::String(isp) => isp,
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
        let isp = isp.to_string();

        let result = Locator {
            ip,
            latitude,
            longitude,
            city,
            region,
            country,
            timezone,
            isp,
        };

        Ok(result)
    }

    async fn ipapico(ip: &str) -> std::result::Result<Self, GeoError> {
        let url = format!("https://ipapi.co/{}/json/", ip);

        let response = match get(&url).await.unwrap().text().await {
            Ok(response) => response,
            Err(_) => {
                return Err(GeoError::HttpError(
                    "Couldn't connect to ipapi.co".to_string(),
                ))
            }
        };

        // Turn the data into parsed_json
        let parsed_json: Value = match serde_json::from_str(&response) {
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
        let isp: String = String::default();

        let result = Locator {
            ip,
            latitude,
            longitude,
            city,
            region,
            country,
            timezone,
            isp,
        };

        Ok(result)
    }
}

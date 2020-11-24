#![feature(ip)]

pub mod ipwhois;
pub mod ip_api;

pub use ipwhois::Locator;
pub use ip_api::Query;

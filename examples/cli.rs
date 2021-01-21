use ipgeolocate::Locator;
use clap::{App, Arg};
use ureq::get;

// A simple CLI application for getting the city and country that an IP is located in.
fn main() {
    let matches = App::new("IP locator")
        .version("0.1.0")
        .author("Grant H. <grantshandy@gmail.com>")
        .about("Finds IP locations")
        .arg(
            Arg::with_name("ADDRESS")
                .help("What IP address to look up, if none are selected your IP address will be chosen")
                .required(false)
                .index(1)
        )
        .arg(
            Arg::with_name("service")
                .long("service")
                .short("s")
                .help("Choose Geolocation API, if not set it defaults to ipapi")
                .required(false)
                .takes_value(true)
                .value_name("SERVICE")
                .possible_value("ipwhois")
                .possible_value("ipapi")
                .possible_value("ipapico")
                .possible_value("freegeoip")
        )
        .get_matches();

    let ip: String = match matches.value_of("ADDRESS") {
        Some(value) => value.to_string(),
        None => {
            match get_ip() {
                Ok(ok) => {
                    println!("No IP address set, using network IP address {}", ok);
                    ok
                },
                Err(error) => {
                    eprintln!("error: {}", error);
                    String::from("ERROR")
                },
            }
        },
    };

    let service = match matches.value_of("service") {
        Some(value) => value,
        None => "ipapi",
    };

    match Locator::get(&ip, service) {
        Ok(ip) => println!("{}: {} - {} ({})", service, ip.ip, ip.city, ip.country),
        Err(error) => println!("Error getting data: {}", error),
    };
}

fn get_ip() -> std::result::Result<String, std::io::Error> {
    let url = format!("http://ifconfig.io/ip");

    let response = get(&url).call();

    if !response.ok() {
        eprintln!("error connecting to ifconfig.io");
    };

    return response.into_string();
}

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use log::{info};

use serde_json::{json};

fn main() {
    let cloud_provider = std::env::args().nth(1).expect("no cloud provider provided");
    let service_type = std::env::args().nth(2).expect("no service type provided");

    let dictionary = match read_dictionary() {
        Ok(inner) => inner,
        Err(_) => panic!("Failure to load dictionary"),
    };

    let cloud_provider_services = dictionary.get(&cloud_provider);

    let services_raw = match cloud_provider_services {
        Some(inner) => inner,
        None => panic!("Cloud Provider is not supoprted with the given dictionary")
    };

    let services_json = json!(services_raw);
    let service_type_uri = &services_json[&service_type].as_str().unwrap();
    println!("URI Value - {:?}", service_type_uri);

    println!("Retrieving template for {}...", service_type);
    debug(format!("Request being directed to {}", service_type_uri));

    make_request(&service_type_uri);
    // TODO - save file into users /terraform/{service_type} directory
    std::process::exit(0);
}

fn read_dictionary() -> Result<HashMap<String, serde_json::Value>, Box<dyn Error>> {
    let dictionary_location = "./dictionary.json";
    let file = File::open(dictionary_location)?;
    let reader = BufReader::new(file);
    let map: HashMap<String, serde_json::Value> = serde_json::from_reader(reader)?;
    Ok(map)
}

fn make_request(uri: &str) {
    let resp = reqwest::blocking::get(uri);
    println!("{:?}", resp);

}

fn debug(msg: String) {
    info!("DEBUG - {}", msg);
}

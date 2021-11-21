use bytes::Bytes;
use log::info;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

static RAW_DICTIONARY: &str = include_str!("dictionary.json");

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
        None => panic!("Cloud Provider is not supoprted with the given dictionary"),
    };

    let services_json = json!(services_raw);
    let service_type_uri = &services_json[&service_type].as_str().unwrap();

    println!("Retrieving template for {}...", service_type);
    debug(format!("Request being directed to {}", service_type_uri));

    let file_bytes = match make_request(&service_type_uri) {
        Ok(inner) => inner,
        Err(_) => panic!("Failure on network request"),
    };

    println!("Writing template file to disk...");
    debug(format!("File bytes length: {}", file_bytes.len()));

    match write_file(file_bytes, &service_type) {
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(1),
    };
}

fn read_dictionary() -> Result<HashMap<String, serde_json::Value>, Box<dyn Error>> {
    let map: HashMap<String, serde_json::Value> = serde_json::from_str(&RAW_DICTIONARY)?;
    Ok(map)
}

fn make_request(uri: &str) -> Result<bytes::Bytes, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(uri).send()?;
    return response.bytes();
}

fn write_file(bytes: Bytes, service_type: &str) -> std::io::Result<()> {
    fs::create_dir_all("./terraform")?;
    let file_name = format!("./terraform/{}.tf", service_type);
    let mut pos = 0;
    let mut buffer = File::create(file_name)?;

    while pos < bytes.len() {
        let bytes_written = buffer.write(&bytes[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}

fn debug(msg: String) {
    info!("DEBUG - {}", msg);
}

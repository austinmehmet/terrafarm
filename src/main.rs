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
    let provider = std::env::args().nth(1).expect("no provider provided");
    let service_type = std::env::args().nth(2).expect("no service type provided");

    let dictionary = match read_dictionary() {
        Ok(inner) => inner,
        Err(_) => panic!("Failure to load dictionary"),
    };

    let services_raw = match dictionary.get(&provider) {
        Some(inner) => inner,
        None => panic!("Cloud Provider is not supoprted with the given dictionary"),
    };

    let services_json = json!(services_raw);

    println!(
        "Retrieving terraform files for {} {}...",
        provider, service_type
    );
    if services_json[&service_type].is_array() {
        let service_type_uris = &services_json[&service_type].as_array().unwrap();
        for uri in service_type_uris.iter() {
            let service_type_uri = &uri.as_str().unwrap();
            retrieve_and_save_file(&service_type_uri)
        }
    } else if services_json[&service_type].is_string() {
        let service_type_uri = &services_json[&service_type].as_str().unwrap();
        retrieve_and_save_file(&service_type_uri)
    } else {
        eprintln!("Invalid type within dictionary - values must be either 'String' or 'Array'");
        std::process::exit(1)
    }
}

fn read_dictionary() -> Result<HashMap<String, serde_json::Value>, Box<dyn Error>> {
    let map: HashMap<String, serde_json::Value> = serde_json::from_str(&RAW_DICTIONARY)?;
    Ok(map)
}

fn retrieve_and_save_file(uri: &str) {
    let file_bytes = match make_request(&uri) {
        Ok(inner) => inner,
        Err(_) => panic!("Failure on network request"),
    };

    let file_name = get_file_name_from_uri(&uri).unwrap();

    match write_file(file_bytes, &file_name) {
        Ok(inner) => inner,
        Err(_) => std::process::exit(1),
    };
}

fn make_request(uri: &str) -> Result<bytes::Bytes, reqwest::Error> {
    debug(format!("Request being directed to {}", uri));
    let client = reqwest::blocking::Client::new();
    let response = client.get(uri).send()?;
    return response.bytes();
}

fn write_file(bytes: Bytes, fname: &str) -> std::io::Result<()> {
    println!("Writing template {} to disk...", fname);

    fs::create_dir_all("./terraform")?;
    let file_name = format!("./terraform/{}", fname);
    let mut pos = 0;
    let mut buffer = File::create(file_name)?;

    debug(format!("File bytes length: {}", bytes.len()));

    while pos < bytes.len() {
        let bytes_written = buffer.write(&bytes[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}

fn get_file_name_from_uri(uri: &str) -> Option<&str> {
    let v: Vec<&str> = uri.split('/').collect();
    return v.last().copied();
}

fn debug(msg: String) {
    info!("DEBUG - {}", msg);
}

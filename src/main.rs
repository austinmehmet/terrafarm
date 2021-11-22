use bytes::Bytes;
use log::info;
use serde_json::json;
use serde_json::Value::Null;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

static RAW_DICTIONARY: &str = include_str!("dictionary.json");

fn main() {
    let matches = App::new("Terrafarm")
        .version("1.0.1")
        .about("A CLI used to query Terraform starter files to assist in getting you off the ground for cloud deployments")
        .subcommand(SubCommand::with_name("get")
            .about("Command for retrieivng terraform files")
            .arg(Arg::with_name("provider")
                .long("provider")
                .short("p")
                .value_name("PROVIDER")
                .help("The provider you want to retrieve from IE aws, azure, gcp, patterns")
                .takes_value(true)
                .required(true)
                .validator(allowed_provider)
            )
            .arg(Arg::with_name("service")
                .long("service")
                .short("s")
                .value_name("SERVICE")
                .help("The service type to retrieve IE dynamodb, lambda, spa")
                .takes_value(true)
                .required(true)
                .validator(allowed_service)
            )
        ).get_matches();

    if let Some(matches) = matches.subcommand_matches("get") {
        handle_get_command(&matches);
    }
}

fn handle_get_command(matches: &ArgMatches) {
    let provider = matches.value_of("provider").unwrap();
    let service_type = matches.value_of("service").unwrap();

    let dictionary = read_dictionary().unwrap();
    let services_json = json!(dictionary.get(provider).unwrap());

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
    } else if services_json[&service_type] == Null {
        eprintln!("Invalid service");
        std::process::exit(1)
    } else {
        eprintln!("Invalid type within dictionary - values must be either 'String' or 'Array'");
        std::process::exit(1)
    }
}

fn allowed_provider(value: String) -> Result<(), String> {
    match read_dictionary().unwrap().get(&value) {
        Some(_) => return Ok(()),
        None => Err(String::from(
            "Must be one of allowed values: [aws, azure, gcp, kubernetes, oci, patterns]",
        )),
    }
}

fn allowed_service(value: String) -> Result<(), String> {
    if RAW_DICTIONARY.contains(&value) {
        return Ok(());
    }
    Err(String::from(
        "Invalid service type - service not found in provided dictionary",
    ))
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

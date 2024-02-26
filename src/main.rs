// To test this run
// cargo run --  -f "foo.csv" "fum.csv" -o "test.csv" union

// cargo run --  --files "../testdata/foo.csv" "../testdata/fum.csv" -o
// ../testdata/union_output.csv union
#![allow(unused_variables)]

extern crate clap;
extern crate csv;

pub mod set_operations;
use std::collections::{HashMap};
use log::{info, debug};
use env_logger::Builder;
use std::error::Error;
use std::process;
use set_operations::union::perform_union;
use set_operations::difference::perform_difference;
use set_operations::intersect::perform_intersect;
use set_operations::clap_config::get_clap_app;
fn perform_operation(operation: fn(Vec<&str>) -> Result<HashMap<String, String>, Box<dyn Error>>, operation_name: &str, files: Vec<&str>) {
    match operation(files) {
        Ok(zet) => {
            // Collect the keys into a vector
            let mut keys: Vec<_> = zet.keys().collect();
            // Sort the keys
            keys.sort();
            // Iterate over the sorted keys and print out the corresponding values
            for key in keys {
                println!("{} result: Key: {:?}, Value: {:?}", operation_name, key, zet.get(key).unwrap());
            }
        }
        Err(e) => {
            eprintln!("Error performing {}: {}", operation_name, e);
            process::exit(1);
        }
    }
}
fn main() {
    debug!("Hello, Hemma på Skeppargatan, we are almost home !");
    // let clap_config_yaml = load_yaml!("clap_config.yml");
    // let app = App::from(clap_config_yaml);
    let app = get_clap_app();
    let matches = app.get_matches();
    // Initialize the logger
    let mut builder = Builder::from_default_env();
    if matches.is_present("debug") {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();

    // Now you can use debug! for debug information and info! for always relevant information
    debug!("Debug mode is on.");
    info!("Info mode is on.");

    if let Some(i) = matches.value_of("OUTPUT") {
        println!("Value for output: {}", i);
    }
    if let Some(i) = matches.value_of("files") {
        let files: Vec<_> = matches.values_of("files").unwrap().collect();
        let files_str = files.join(", ");
        debug!("Value for files: {}", files_str);
        match matches.subcommand() {
            ("UNION", Some(_)) => {
                perform_operation(perform_union, "Union", files.iter().map(AsRef::as_ref).collect());
            }
            ("INTERSECT", Some(_)) => {
                perform_operation(perform_intersect, "Intersection", files.iter().map(AsRef::as_ref).collect());
            }
            ("DIFFERENCE", Some(_)) => {
                perform_operation(perform_difference, "Difference", files.iter().map(AsRef::as_ref).collect());
            }
            _ => println!("No valid subcommand was used"),
        }
    }
}



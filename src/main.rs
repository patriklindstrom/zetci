// To test this run
// cargo run --  -f "foo.csv" "fum.csv" -o "test.csv" union

// cargo run --  --files "../testdata/foo.csv" "../testdata/fum.csv" -o ../testdata/union_output.csv union
#![allow(unused_variables)]

extern crate csv;

pub mod logo;
pub mod set_operations;

use crate::set_operations::clap_config::cli;
use env_logger::Builder;
use log::{debug, info};
use logo::logo;
use set_operations::diffa::perform_diffa;
use set_operations::intersect::perform_intersect;
use set_operations::union::perform_union;
use set_operations::xor::perform_xor;
use std::collections::HashMap;
use std::error::Error;
use std::{env, process};

fn perform_operation(
    operation: impl FnOnce(Vec<&String>) -> Result<HashMap<String, String>, Box<dyn Error>>,
    operation_name: &str,
    files: Vec<&String>,
) {
    match operation(files) {
        Ok(zet) => {
            // Collect the keys into a vector
            let mut keys: Vec<_> = zet.keys().collect();
            // Sort the keys
            keys.sort();
            // Iterate over the sorted keys and print out the corresponding values
            for key in keys {
                println!(
                    "{} result: Key: {:?}, Value: {:?}",
                    operation_name,
                    key,
                    zet.get(key).unwrap()
                );
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

    let matches = cli().get_matches();
    // Initialize the logger
    let mut builder = Builder::from_default_env();
    /*    if matches.args_present("debug"){
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }*/
    builder.init();
    let cwd = env::current_dir().expect("Failed to get current directory");
    println!("Current working directory: {}", cwd.display());
    // Now you can use debug! for debug information and info! for always relevant information
    debug!("Debug mode is on.");
    info!("Info mode is on.");

    /*    if let Some(i) = matches.value_of("OUTPUT") {
        println!("Value for output: {}", i);
    }*/
    if let Some(files) = matches.get_many::<String>("files") {
        let files_vec: Vec<&String> = files.collect(); // Collect into a Vec<&String>
        let files_str = files_vec
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>()
            .join(", ");
        println!("Files: {}", files_str);
        debug!("Value for files: {}", files_str);

        match matches.subcommand() {
            Some(("union", sub_matches)) => {
                perform_operation(perform_union, "Union", files_vec);
            }
            Some(("intersect", sub_matches)) => {
                perform_operation(perform_intersect, "Intersection", files_vec.clone());
            }
            Some(("diffa", sub_matches)) => {
                perform_operation(perform_diffa, "Difference", files_vec.clone());
            }
            Some(("xor", sub_matches)) => {
                let strategy = sub_matches
                    .get_one::<String>("value-strategy")
                    .map(|s| s.as_str())
                    .unwrap_or("first");
                
                perform_operation(
                    |files| perform_xor(files, strategy),
                    "Xor",
                    files_vec.clone()
                );
            }
            _ => println!("No valid subcommand was used"),
        }
    }
    match matches.subcommand() {
        Some(("about", sub_matches)) => {
            println!("{}", logo());
        }
        _ => println!("No valid subcommand was used"),
    }
}

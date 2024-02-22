// To test this run
// cargo run --  -f "foo.csv" "fum.csv" -o "test.csv" union

// cargo run --  --files "../testdata/foo.csv" "../testdata/fum.csv" -o
// ../testdata/union_output.csv union
#![allow(unused_variables)]

extern crate clap;
extern crate csv;

use clap::{load_yaml, App};
use std::collections::HashMap;
use log::{info, debug};
use env_logger::Builder;
use std::env;
use std::error::Error;
use std::fs::File;
use std::process;
const KEY_COLUMN: usize = 0;
const HAS_HEADERS: bool = false;
fn read_data_file(file_path: String, key_column: usize) -> Result<HashMap<String, String>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    // let file_path: String = "/home/patrik/git/zet-cmder/testdata/fee.csv".to_string();
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(HAS_HEADERS)
        .from_reader(file);
    let mut set: HashMap<String, String> = HashMap::new();
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        debug!("{:?}", record);
        debug!("{:?}", record.get(key_column));
        let value: String = record.iter().collect::<Vec<&str>>().join(",");
        set.insert(record.get(key_column).unwrap().to_string(), value);
    }
    Ok(set)
}

// perform_union should return the zets variable
fn perform_union(files: Vec<&str>) -> Result<HashMap<String, String>, Box<dyn Error>> {
    // Placeholder for actual union logic
    info!("Performing union operation...");
    let mut zets: HashMap<String, String> = HashMap::new();
    for f in files {
        let dset = read_data_file(f.to_string(),KEY_COLUMN).expect("Cant handle file");
        for (key, value) in &dset {
            if !zets.contains_key(key) {
                zets.insert(key.clone(), value.clone());
            }
        }
        info!("Processed file: {}", f);
    }
    info!("Number of Hashmaps are {:?}", zets.len());
    Ok(zets)
}

fn perform_intersect(files: Vec<&str>) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    // Placeholder for actual intersect logic
    info!("Performing intersect operation...");
    // Similar to perform_union but for intersection
    let mut zets: Vec<HashMap<String, String>> = Vec::with_capacity(files.len());
    for f in files {
        let dset = read_data_file(f.to_string(), KEY_COLUMN).expect("Cant handle file");
        zets.push(dset);
        // Actual intersect  operation would go here
        info!("Processed file: {}", f);
    }
    info!("Number of Hashmaps are {:?}", zets.len());
    Ok(zets)
}

fn get_current_dir() -> String {
    let path = env::current_dir().unwrap();
    return path.display().to_string();
}

fn main() {
    debug!("Hello, Hemma på Skeppargatan !");
    debug!("The current directory is {}", get_current_dir());
    let clap_config_yaml = load_yaml!("clap_config.yml");
    let app = App::from(clap_config_yaml);
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
            ("UNION", Some(sub_m)) => {
                match perform_union(files.iter().map(AsRef::as_ref).collect()) {
                    Ok(zets) => {
                        // Collect the keys into a vector
                        let mut keys: Vec<_> = zets.keys().collect();
                        // Sort the keys
                        keys.sort();
                        // Iterate over the sorted keys and print out the corresponding values
                        for key in keys {
                            println!("Union result: Key: {:?}, Value: {:?}", key, zets.get(key).unwrap());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error performing union: {}", e);
                        process::exit(1);
                    }
                }
            }
            ("INTERSECT", Some(sub_m)) => {
                if let Err(e) = perform_intersect(files.iter().map(AsRef::as_ref).collect()) {
                    eprintln!("Error performing intersection: {}", e);
                    process::exit(1);
                }
            }
            _ => println!("No valid subcommand was used"),
        }
    }
}

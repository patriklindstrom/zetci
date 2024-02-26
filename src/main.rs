// To test this run
// cargo run --  -f "foo.csv" "fum.csv" -o "test.csv" union

// cargo run --  --files "../testdata/foo.csv" "../testdata/fum.csv" -o
// ../testdata/union_output.csv union
#![allow(unused_variables)]

extern crate clap;
extern crate csv;

use clap::{load_yaml, App};
use std::collections::{HashMap, HashSet};
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

// A or B
// perform_union should return the zets variable
fn perform_union(files: Vec<&str>) -> Result<HashMap<String, String>, Box<dyn Error>> {
    // Placeholder for actual union logic
    info!("Performing union operation...");
    let mut zet: HashMap<String, String> = HashMap::new();
    for f in files {
        info!("Opening file: {}", f);
        let d_set = read_data_file(f.to_string(), KEY_COLUMN).expect("Cant handle file");
        for (key, value) in &d_set {
            if !zet.contains_key(key) {
                zet.insert(key.clone(), value.clone());
            }
        }
        info!("Processed file: {}", f);
    }

    info!("Number of Hashmaps are {:?}", zet.len());
    Ok(zet)
}

/* A and B
We first create a counts hashmap to keep track of the count of each key across all files.
 Then, we read each file into the zet hashmap and increment the count of each key in the counts hashmap.
 Finally, we retain only the keys in zet whose count is equal to the number of files.
 This ensures that only the keys that exist in all files are retained.*/
fn perform_intersect(files: Vec<&str>) -> Result<HashMap<String, String>, Box<dyn Error>> {
    info!("Performing intersect operation...");
    info!("Current directory file: {}", get_current_dir());

    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut zet: HashMap<String, String> = HashMap::new();
    for f in &files {
        info!("Opening file: {}", f);
        let d_set = read_data_file(f.to_string(), KEY_COLUMN).expect("Cant handle file");
        for (key, value) in d_set {
            *counts.entry(key.clone()).or_insert(0) += 1;
            zet.insert(key, value);
        }
    }
    zet.retain(|key, _| counts.get(key) == Some(&files.len()));

    Ok(zet)
}
// not A and B
fn perform_difference(files: Vec<&str>) -> Result<HashMap<String, String>, Box<dyn Error>> {
    info!("Performing difference set operation...");
    let a_file = &files[0];
    info!("Current directory file: {}", get_current_dir());
    info!("Opening file: {}", a_file);
    let mut a_zet: HashMap<String, String> = read_data_file(a_file.to_string(), KEY_COLUMN).expect("Cant handle file");
    for f in files.iter().skip(1) {
        println!("Opening file: {}", f);
        let d_set = read_data_file(f.to_string(), KEY_COLUMN).expect("Cant handle file");
        let d_keys: HashSet<_> = d_set.keys().map(|k| k.clone()).collect();
        a_zet.retain(|key, _| !d_keys.contains(key));
        info!("Processed file: {}", f);
    }
    Ok(a_zet)
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

                    Ok(zet) => {
                        // Collect the keys into a vector
                        let mut keys: Vec<_> = zet.keys().collect();

                        // Sort the keys
                        keys.sort();
                        // Iterate over the sorted keys and print out the corresponding values
                        for key in keys {
                            println!("Union result: Key: {:?}, Value: {:?}", key, zet.get(key).unwrap());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error performing union: {}", e);
                        process::exit(1);
                    }
                }
            }
            ("INTERSECT", Some(sub_m)) => {
                match perform_intersect(files.iter().map(AsRef::as_ref).collect()) {
                    Ok(zet) => {
                        // Collect the keys into a vector
                        let mut keys: Vec<_> = zet.keys().collect();
                        // Sort the keys
                        keys.sort();
                        // Iterate over the sorted keys and print out the corresponding values
                        for key in keys {
                            println!("Intersection result: Key: {:?}, Value: {:?}", key, zet.get(key).unwrap());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error performing intersection: {}", e);
                        process::exit(1);
                    }
                }

            }
            ("DIFFERENCE", Some(sub_m)) => {
                match perform_difference(files.iter().map(AsRef::as_ref).collect()) {
                    Ok(zet) => {
                        // Collect the keys into a vector
                        let mut keys: Vec<_> = zet.keys().collect();
                        // Sort the keys
                        keys.sort();
                        // Iterate over the sorted keys and print out the corresponding values
                        for key in keys {
                            println!("Left join result: Key: {:?}, Value: {:?}", key, zet.get(key).unwrap());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error performing Left Join: {}", e);
                        process::exit(1);
                    }
                }

            }
            _ => println!("No valid subcommand was used"),
        }
    }
}



// To test this run
// cargo run --  -f "foo.csv" "fum.csv" -o "test.csv" union

// cargo run --  --files "../testdata/foo.csv" "../testdata/fum.csv" -o
// ../testdata/union_output.csv union
#![allow(unused_variables)]

extern crate clap;
extern crate csv;

use clap::{load_yaml, App};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::process;

fn read_data_file(file_path: String) -> Result<HashMap<String, String>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    // let file_path: String = "/home/patrik/git/zet-cmder/testdata/fee.csv".to_string();
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut set: HashMap<String, String> = HashMap::new();
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
        println!("{:?}", record.get(0));
        let value: String = record.as_slice().to_string();
        set.insert(record.get(0).unwrap().to_string(), value);
    }
    Ok(set)
}

// perform_union should return the zets variable
fn perform_union(files: Vec<&str>) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    // Placeholder for actual union logic
    println!("Performing union operation...");
    let mut zets: Vec<HashMap<String, String>> = Vec::with_capacity(files.len());
    for f in files {
        let dset = read_data_file(f.to_string()).expect("Cant handle file");
        zets.push(dset);
        // Actual union operation would go here
        println!("Processed file: {}", f);
    }
    println!("Number of Hashmaps are {:?}", zets.len());
    Ok(zets)
}

fn perform_intersect(files: Vec<&str>) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    // Placeholder for actual intersect logic
    println!("Performing intersect operation...");
    // Similar to perform_union but for intersection
    let mut zets: Vec<HashMap<String, String>> = Vec::with_capacity(files.len());
    for f in files {
        let dset = read_data_file(f.to_string()).expect("Cant handle file");
        zets.push(dset);
        // Actual intersect  operation would go here
        println!("Processed file: {}", f);
    }
    println!("Number of Hashmaps are {:?}", zets.len());
    Ok(zets)
}

fn get_current_dir() -> String {
    let path = env::current_dir().unwrap();
    return path.display().to_string();
}

fn main() {
    println!("Hello, Hemma på Skeppargatan !");
    println!("The current directory is {}", get_current_dir());
    let clap_config_yaml = load_yaml!("clap_config.yml");
    let app = App::from(clap_config_yaml);
    let matches = app.get_matches();
    if let Some(i) = matches.value_of("OUTPUT") {
        println!("Value for output: {}", i);
    }
    if let Some(i) = matches.value_of("files") {
        let files: Vec<_> = matches.values_of("files").unwrap().collect();
        let files_str = files.join(", ");
        println!("Value for files: {}", files_str);
        match matches.subcommand() {
            ("UNION", Some(sub_m)) => {
                match perform_union(files.iter().map(AsRef::as_ref).collect()) {
                    Ok(zets) => {
                        // loop over the zets and print them out
                        for z in zets.iter() {
                            println!("Union result: {:?}", z);
                        }
                        //
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

// To test this run
// cargo run --  -f "foo.csv" "fum.csv" -o "test.csv" union

// cargo run --  --files "../testdata/foo.csv" "../testdata/fum.csv" -o
// ../testdata/union_output.csv union

extern crate clap;
extern crate csv;
use clap::{load_yaml, App};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::process;

fn read_data_file(file_path: String) -> Result<HashMap<String,String>, Box<dyn Error>> {
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
fn perform_union(files: Vec<&str>) -> Result<(), Box<dyn Error>> {
    // Placeholder for actual union logic
    println!("Performing union operation...");
    for f in files {
        let data = read_data_file(f.to_string())?;
        // Actual union operation would go here
        println!("Processed file: {}", f);
    }
    Ok(())
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
    if matches.is_present("FILES") {
        // "$  -f filename1.csv filename2.csv" was run
        let files: Vec<_> = matches.values_of("FILES").unwrap().collect();
        let mut zets = Vec::with_capacity(files.len());
        for f in files {
            println!("file : {}", f);
            let dset=  read_data_file(f.to_string()).expect("Cant handle file") ;
            println!("Hashmap's_length_comes for {} is {:?}",f.to_string(),dset.len());
            zets.push(dset);
            //match dset {
            //    Ok(res_set)=> println!("Hashmap's_length_comes out to_be {:?}",res_set.len()),
            //    Err(e)=> println!("Error"),
            //}
        }
        println!("Number of Hashmapis is {:?}",zets.len());
    }

    if matches.is_present("UNION") {
        println!("operation is Union")
    }
}

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

fn read_data_file(file_path: String) -> Result<(), Box<dyn Error>> {
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
    Ok(())
}

fn get_current_dir() -> String {
    let path = env::current_dir().unwrap();
    return path.display().to_string();
}
fn main() {
    println!("Hello,funäsdalen !");
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
        for f in files {
            println!("file : {}", f);
            if let Err(err) = read_data_file(f.to_string()) {
                println!("{}", err);
                process::exit(1);
            }
            match matches.subcommand_name() {
                // TODO: Add function that takes array of hashmaps with data from files and
                // performs the operation union on them. move function later to library.
                // TODO: The array of function needs to be a struct perhaps with meta data about
                // the data like which one is the biggest, cardinality and perhaps others so the
                // rudimentary queriy optimizer gets relevant info.
                Some("union") => println!("'zet-cmder union' was used"),
                Some("intersect") => println!("'zet-cmder intersect' was used"),
                None => println!("No operator like union or intersect was used"),
                _ => println!(
                    "unknown subcommand used {:?}. Use --help to see valid operators ",
                    matches.subcommand_name()
                ),
            }
        }
    }
}

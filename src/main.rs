// To test this run
// cargo run --  -f "foo.csv" "fum.csv" -o "test.csv" --union

// cargo run --  --files "../testdata/foo.csv" "../testdata/fum.csv" -o
// ../testdata/union_output.csv union

extern crate clap;
extern crate csv;
use clap::{load_yaml, App};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;

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
        let files: Vec<_> = matches.values_of("FILES").unwrap().collect();
        let mut zets = Vec::with_capacity(files.len());
        for f in files {
            println!("file : {}", f);
            let dset = read_data_file(f.to_string()).expect("Cant handle file");
            println!(
                "Hashmap's_length_comes for {} is {:?}",
                f.to_string(),
                dset.len()
            );
            zets.push(dset);
        }
        println!("Number of Hashmapis is {:?}", zets.len());
        if matches.is_present("UNION") {
            println!("operation is Union");
            // Sort Descending on size
            zets.sort_by(|a, b| b.len().cmp(&a.len()));
            let first_z = zets.first();
            match first_z {
                Some(r) => println!("Length of first_z is {:?}", r.len()),
                None => println!("Error in first_z"),
            }
            let z_union_canditates = &zets[1..];
            for z in z_union_canditates.iter() {
                println!("Nr of Rows in Hashmap is {:?}", z.len());
                for r in z {
                    // now check if this r row exist in
                    println!("r exist or not it is now: {:?}", r)
                }
            }
        }
    }
}

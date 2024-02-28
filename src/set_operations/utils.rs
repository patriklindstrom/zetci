use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use log::debug;

const HAS_HEADERS: bool = false;

pub fn read_data_file(file_path: String, key_column: usize) -> Result<HashMap<String, String>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    // let file_path: String = "/home/patrik/git/zetci/testdata/fee.csv".to_string();
    debug!("The current directory is {}", get_current_dir());
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

pub fn get_current_dir() -> String {
    let path = env::current_dir().unwrap();
    return path.display().to_string();
}
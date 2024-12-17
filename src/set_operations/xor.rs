use crate::set_operations::utils::read_data_file;
use log::info;
use std::collections::HashMap;
use std::error::Error;

const KEY_COLUMN: usize = 0;

/// Performs the XOR (symmetric difference) operation across multiple files.
/// Includes keys that appear in an odd number of files.
pub fn perform_xor(files: Vec<&String>) -> Result<HashMap<String, String>, Box<dyn Error>> {
    info!("Performing XOR operation...");

    let mut counts: HashMap<String, usize> = HashMap::new(); // Tracks how many times each key appears
    let mut zet: HashMap<String, String> = HashMap::new(); // Holds the final XOR results

    // Iterate through each file and read the data
    for f in &files {
        info!("Opening file: {}", f);
        let d_set = read_data_file(f.to_string(), KEY_COLUMN).expect("Cannot handle file");

        // Update counts and zet for each key-value pair
        for (key, value) in d_set {
            *counts.entry(key.clone()).or_insert(0) += 1; // Increment key count
            zet.entry(key).or_insert(value); // Insert the key-value pair into zet if not already present
        }
    }

    // Retain only keys that appear in an odd number of files
    zet.retain(|key, _| counts.get(key).map(|count| count % 2 == 1).unwrap_or(false));

    info!("Number of keys in XOR result: {}", zet.len());
    Ok(zet)
}

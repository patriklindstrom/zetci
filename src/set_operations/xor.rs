use crate::set_operations::utils::read_data_file;
use log::info;
use std::collections::HashMap;
use std::error::Error;

const KEY_COLUMN: usize = 0;

#[derive(Debug, PartialEq)]
pub enum ValueStrategy {
    First,
    Last,
    Concat,
}

impl From<&str> for ValueStrategy {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "last" => ValueStrategy::Last,
            "concat" => ValueStrategy::Concat,
            _ => ValueStrategy::First, // default to First for any unknown strategy
        }
    }
}

/// Performs the XOR (symmetric difference) operation across multiple files.
/// Includes keys that appear in an odd number of files.
/// 
/// The value-strategy parameter controls how values are handled when a key appears multiple times:
/// - "first": Keep the value from the first file where the key appears (default)
/// - "last": Keep the value from the last file where the key appears
/// - "concat": Concatenate all values with " || " separator
pub fn perform_xor(
    files: Vec<&String>,
    strategy: &str
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    info!("Performing XOR operation with strategy: {}", strategy);
    let strategy = ValueStrategy::from(strategy);
    
    let mut counts: HashMap<String, usize> = HashMap::new(); // Tracks how many times each key appears
    let mut zet: HashMap<String, String> = HashMap::new(); // Holds the final XOR results

    // Iterate through each file and read the data
    for f in &files {
        info!("Opening file: {}", f);
        let d_set = read_data_file(f.to_string(), KEY_COLUMN)?;
        
        // Update counts and zet for each key-value pair
        for (key, value) in d_set {
            *counts.entry(key.clone()).or_insert(0) += 1; // Increment key count
            
            match strategy {
                ValueStrategy::First => {
                    zet.entry(key).or_insert(value);
                },
                ValueStrategy::Last => {
                    zet.insert(key, value);
                },
                ValueStrategy::Concat => {
                    zet.entry(key)
                       .and_modify(|v| {
                           v.push_str(" || ");
                           v.push_str(&value);
                       })
                       .or_insert(value);
                }
            }
        }
    }

    // Retain only keys that appear in an odd number of files
    zet.retain(|key, _| counts.get(key).map(|count| count % 2 == 1).unwrap_or(false));

    info!("Number of keys in XOR result: {}", zet.len());
    Ok(zet)
}

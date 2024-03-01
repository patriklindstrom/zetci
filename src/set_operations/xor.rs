
use std::collections::HashMap;
use std::error::Error;
use log::info;
use crate::set_operations::utils::{read_data_file};
const KEY_COLUMN: usize = 0;
static ONE: usize = 1;
static NO_FOUND_IN_FILES: Option<&usize> = Some(&ONE);
// A or B
// perform_union should return the zets variable
pub fn perform_xor(files: Vec<&String>) -> Result<HashMap<String, String>, Box<dyn Error>> {
    // Placeholder for actual union logic
    info!("Performing xor operation...");
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
    zet.retain(|key, _| counts.get(key) == NO_FOUND_IN_FILES);

    Ok(zet)
}
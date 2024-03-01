
use std::collections::HashMap;
use std::error::Error;
use log::info;
use crate::set_operations::utils::{read_data_file};
const KEY_COLUMN: usize = 0;

// A or B
// perform_union should return the zets variable
pub fn perform_union(files: Vec<&String>) -> Result<HashMap<String, String>, Box<dyn Error>> {
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
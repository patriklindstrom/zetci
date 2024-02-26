use std::collections::HashMap;
use std::error::Error;
use log::info;
use crate::set_operations::utils::{read_data_file};
const KEY_COLUMN: usize = 0;
/* A and B
We first create a counts hashmap to keep track of the count of each key across all files.
 Then, we read each file into the zet hashmap and increment the count of each key in the counts hashmap.
 Finally, we retain only the keys in zet whose count is equal to the number of files.
 This ensures that only the keys that exist in all files are retained.*/
pub fn perform_intersect(files: Vec<&str>) -> Result<HashMap<String, String>, Box<dyn Error>> {
    info!("Performing intersect operation...");
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
use std::collections::HashMap;
use std::error::Error;
use crate::set_operations::utils::read_data_file;

const KEY_COLUMN: usize = 0;

pub fn perform_left_join(files: Vec<&String>) -> Result<HashMap<String, String>, Box<dyn Error>> {
    if files.len() < 2 {
        return Err("Left Join requires at least two files.".into());
    }

    let left_file = &files[0];
    let right_files = &files[1..];

    let left_map = read_data_file(left_file.to_string(), KEY_COLUMN)?;
    let mut joined: HashMap<String, String> = HashMap::new();

    // Load all right-side maps into one
    let mut right_maps: HashMap<String, String> = HashMap::new();
    for f in right_files {
        let map = read_data_file(f.to_string(), KEY_COLUMN)?;
        for (k, v) in map {
            right_maps.insert(k, v);
        }
    }

    // Join left with right
    for (key, left_value) in &left_map {
        if let Some(right_value) = right_maps.get(key) {
            joined.insert(key.clone(), format!("{},{}", left_value, right_value));
        } else {
            joined.insert(key.clone(), format!("{},{}", left_value, ""));
        }
    }

    Ok(joined)
}

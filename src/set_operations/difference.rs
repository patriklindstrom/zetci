use std::collections::{HashMap, HashSet};
use std::error::Error;
use log::info;
use crate::set_operations::utils::{get_current_dir, read_data_file};
const KEY_COLUMN: usize = 0;
// not A and B
pub fn perform_difference(files: Vec<&str>) -> Result<HashMap<String, String>, Box<dyn Error>> {
    info!("Performing difference set operation...");
    let a_file = &files[0];
    info!("Current directory file: {}", get_current_dir());
    info!("Opening file: {}", a_file);
    let mut a_zet: HashMap<String, String> = read_data_file(a_file.to_string(), KEY_COLUMN).expect("Cant handle file");
    for f in files.iter().skip(1) {
        println!("Opening file: {}", f);
        let d_set = read_data_file(f.to_string(), KEY_COLUMN).expect("Cant handle file");
        let d_keys: HashSet<_> = d_set.keys().map(|k| k.clone()).collect();
        a_zet.retain(|key, _| !d_keys.contains(key));
        info!("Processed file: {}", f);
    }
    Ok(a_zet)
}
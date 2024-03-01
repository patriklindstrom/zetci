
// tests/set_operations_test.rs
use std::collections::HashMap;
use zetci::set_operations::intersect::perform_intersect;
// Add reference to the pub function pub fn perform_union in the module under src/set_operations/union.rs
use zetci::set_operations::union::perform_union;
use zetci::set_operations::diffa::perform_diffa;
use zetci::set_operations::xor::perform_xor;
#[test]
fn test_perform_union() {
    let files = vec!["./testdata/fee.csv","./testdata/foo.csv", "./testdata/fum.csv"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let files_ref = files.iter().collect::<Vec<&String>>();
    let result = perform_union(files_ref).unwrap();
let expected: HashMap<String, String> = [
    ("0", "0,Lilith,Larsson,6000 m,16 min 2 s,Bare"),
    ("1", "1,Adam,Svensson,3000 m,12 min 30 s,Nike"),
    ("2", "2,Bertil,Svensson,6000 m,27 min 5 s,Adidas"),
    ("3", "3,Cecilia,Svensson,2000 m,9 min 15 s,Nike"),
    ("4", "4,Olle,Lindström,3000,10 min 22 s,Na"),
    ("5", "5,Emma,Olsson,2000 m,11 min 25 s,Nike"),
    ("6", "6,Anna,Lindström,1000 m,8 min 31 s,Adidas"),
    ("7", "7,Hampus,Olsson,1000 m,8 min 15 s,Nike"),
    ("8","8,Zorro,Collie,1000 m,3 min 11 s,Tass"),
    ("9","9,Jane,Lindstrom,500 m,4 min 47 s,Sandal"),
].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    assert_eq!(result, expected, "The union of the three files should be all keys from 0 to 7");
}
#[test]
fn test_perform_intersect() {
    let files = vec!["./testdata/fee.csv","./testdata/foo.csv"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let files_ref = files.iter().collect::<Vec<&String>>();
    let result = perform_intersect(files_ref).unwrap();
    let expected: HashMap<String, String> = [
        ("4", "4,Olle,Lindström,3000,10 min 22 s,Na"),
        ("5", "5,Emma,Olsson,2000 m,11 min 25 s,Nike"),
    ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    assert_eq!(result, expected, "The intersection of the three files should be 4 and 5");
}

#[test]
fn test_perform_diffa() {
    let files = vec!["./testdata/fum.csv","./testdata/fee.csv"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let files_ref = files.iter().collect::<Vec<&String>>();
    let result = perform_diffa(files_ref).unwrap();
    let expected: HashMap<String, String> = [
        ("1", "1,Adam,Svensson,3000 m,12 min 30 s,Nike"),
        ("2", "2,Bertil,Svensson,6000 m,27 min 5 s,Adidas"),
        ("3", "3,Cecilia,Svensson,2000 m,9 min 15 s,Nike"),
        ("8","8,Zorro,Collie,1000 m,3 min 11 s,Tass"),
    ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    assert_eq!(result, expected, "The difference of the two files fum.csv and fee.csv should be the keys 1,2,3");
}
#[test]
fn test_perform_xor() {
    let files = vec!["./testdata/fum.csv","./testdata/fee.csv","./testdata/foo.csv"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let files_ref = files.iter().collect::<Vec<&String>>();
    let result = perform_xor(files_ref).unwrap();
    let expected: HashMap<String, String> = [
        ("7","7,Hampus,Olsson,1000 m,8 min 15 s,Nike"),
        ("8","8,Zorro,Collie,1000 m,3 min 11 s,Tass"),
        ("9","9,Jane,Lindstrom,500 m,4 min 47 s,Sandal"),
    ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    assert_eq!(result, expected, "The Exclusive Or of the three files fum.csv fee.csv and foo.csv should be the keys 7,8,9");
}
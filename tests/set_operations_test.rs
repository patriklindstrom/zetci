
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
fn test_perform_xor_two_sets() {
    let files = vec!["./testdata/fum.csv", "./testdata/fee.csv"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let files_ref = files.iter().collect::<Vec<&String>>();
    let result = perform_xor(files_ref, "first").unwrap();

    // Convert result HashMap into a sorted Vec<(String, String)>
    let mut result_vec: Vec<(String, String)> = result.into_iter().collect();
    result_vec.sort_by(|a, b| a.0.cmp(&b.0)); // Sort by key

    // Define the expected HashMap and convert it to a sorted Vec<(String, String)>
    let expected: HashMap<String, String> = [
        ("1", "1,Adam,Svensson,3000 m,12 min 30 s,Nike"),
        ("2", "2,Bertil,Svensson,6000 m,27 min 5 s,Adidas"),
        ("3", "3,Cecilia,Svensson,2000 m,9 min 15 s,Nike"),
        ("7", "7,Hampus,Olsson,1000 m,8 min 15 s,Nike"),
        ("8", "8,Zorro,Collie,1000 m,3 min 11 s,Tass"),
    ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let mut expected_vec: Vec<(String, String)> = expected.into_iter().collect();
    expected_vec.sort_by(|a, b| a.0.cmp(&b.0)); // Sort by key

    // Compare the sorted vectors
    assert_eq!(result_vec, expected_vec, "The Exclusive Or of the two files fum.csv fee.csv ");
}

#[test]
fn test_perform_xor_three_files() {
    let files = vec!["./testdata/fum.csv","./testdata/fee.csv","./testdata/foo.csv"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let files_ref = files.iter().collect::<Vec<&String>>();
    let result = perform_xor(files_ref, "first").unwrap();
    let expected: HashMap<String, String> = [
        ("7","7,Hampus,Olsson,1000 m,8 min 15 s,Nike"),
        ("8","8,Zorro,Collie,1000 m,3 min 11 s,Tass"),
        ("9","9,Jane,Lindstrom,500 m,4 min 47 s,Sandal"),
        ("5", "5,Emma,Olsson,2000 m,11 min 25 s,Nike"),
        ("4", "4,Olle,Lindström,3000,10 min 22 s,Na"),
    ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    assert_eq!(result, expected, "The Exclusive Or of the three files fum.csv fee.csv and foo.csv should be the keys 7,8");
}
#[test]
fn test_perform_xor_four_sets() {
    let files = vec!["./testdata/fum.csv","./testdata/fee.csv","./testdata/foo.csv","./testdata/smell.csv"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let files_ref = files.iter().collect::<Vec<&String>>();
    let result = perform_xor(files_ref, "first").unwrap();
    let expected: HashMap<String, String> = [
        ("0", "0,Lilith,Larsson,6000 m,16 min 2 s,Bare"),  // From `fum.csv`
        ("1", "1,Adam,Svensson,3000 m,12 min 30 s,Nike"),  // From `fum.csv`
        ("2", "2,Bertil,Svensson,6000 m,27 min 5 s,Adidas"), // From `fum.csv`
        ("3", "3,Cecilia,Svensson,2000 m,9 min 15 s,Nike"),  // From `fum.csv`
        ("6", "6,Anna,Lindström,1000 m,8 min 31 s,Adidas"), // From `fum.csv`
        ("7", "7,Hampus,Olsson,1000 m,8 min 15 s,Nike"),   // From `fee.csv`
        ("9", "9,Jane,Lindstrom,500 m,4 min 47 s,Sandal"),  // From `foo.csv`
        ("11", "11,Joanna,Lindström,1000 m,12 min 31 s,High Heel"), // From `smell.csv`
    ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    assert_eq!(result, expected, "The Exclusive Or of the three files fum.csv fee.csv and foo.csv should be the keys 7,8,9");
}
    

    #[test]
    fn test_perform_xor_with_first_strategy() {
    let files = vec!["./testdata/fum.csv", "./testdata/foo.csv"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let files_ref = files.iter().collect::<Vec<&String>>();
    
    let result = perform_xor(files_ref, "first").unwrap();
    
    // Key "0" appears in both fum.csv and foo.csv with different values
    // With "first" strategy, we expect the value from fum.csv
    assert_eq!(
        result.get("0").unwrap(),
        "0,Lilith,Larsson,6000 m,16 min 2 s,Bare"
    );
    }
    
    #[test]
    fn test_perform_xor_with_last_strategy() {
        let files = vec!["./testdata/fum.csv", "./testdata/foo.csv"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();
        let files_ref = files.iter().collect::<Vec<&String>>();
        
        let result = perform_xor(files_ref, "last").unwrap();
        
        // Checking keys that appear in only one file
        // Key "0" appears only in fum.csv
        assert_eq!(
            result.get("0").unwrap(),
            "0,Lilith,Larsson,6000 m,16 min 2 s,Bare"
        );
        
        // Key "9" appears only in foo.csv
        assert_eq!(
            result.get("9").unwrap(),
            "9,Jane,Lindstrom,500 m,4 min 47 s,Sandal"
        );
        
        // Make sure keys in both files (1,2,3,4,5) are not in the result
        assert!(result.get("1").is_none());
        assert!(result.get("2").is_none());
        assert!(result.get("3").is_none());
        assert!(result.get("4").is_none());
        assert!(result.get("5").is_none());
    }
    
    #[test]
    fn test_perform_xor_with_concat_strategy() {
        let files = vec!["./testdata/fum.csv", "./testdata/foo.csv"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();
        let files_ref = files.iter().collect::<Vec<&String>>();
        
        let result = perform_xor(files_ref, "concat").unwrap();
        
        // Check keys that appear in only one file
        assert_eq!(
            result.get("0").unwrap(),
            "0,Lilith,Larsson,6000 m,16 min 2 s,Bare"
        );
        
        assert_eq!(
            result.get("9").unwrap(),
            "9,Jane,Lindstrom,500 m,4 min 47 s,Sandal"
        );
    }
    
    #[test]
    fn test_perform_xor_invalid_strategy() {
        let files = vec!["./testdata/fum.csv", "./testdata/foo.csv"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();
        let files_ref = files.iter().collect::<Vec<&String>>();
        
        // Invalid strategy should default to "first"
        let result = perform_xor(files_ref, "invalid_strategy").unwrap();
        
        // Check a key that appears in only one file
        assert_eq!(
            result.get("0").unwrap(),
            "0,Lilith,Larsson,6000 m,16 min 2 s,Bare"
        );
    }



use std::collections::HashMap;
use tempfile::tempdir;
use zetci::set_operations::union::perform_union;
use zetci::set_operations::intersect::perform_intersect;
use zetci::set_operations::diffa::perform_diffa;
use zetci::set_operations::xor::perform_xor;
use tests_support::gen::{write_csv, GenCfg};

mod tests_support;

fn rows_from_env(default: u64) -> u64 {
    std::env::var("ZETCI_LARGE_ROWS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(default)
}

#[test]
fn large_union_intersect_xor_smoke() {
    let rows = rows_from_env(50_000); // bump with: ZETCI_LARGE_ROWS=500000 cargo test --release -- large_union_intersect_xor_smoke -- --nocapture
    let dir = tempdir().unwrap();
    let f1 = dir.path().join("a.csv");
    let f2 = dir.path().join("b.csv");

    // File A: keys [0..rows), File B: overlap plus different region
    write_csv(&f1, GenCfg {
        rows,
        key_start: 0,
        dup_prob: 0.01,
        conflict_prob: 0.05,
        long_field_prob: 0.001,
        unicode: true,
        crlf: false,
        seed: 42,
    }).unwrap();

    write_csv(&f2, GenCfg {
        rows,
        key_start: rows / 2,             // 50% overlap
        dup_prob: 0.02,                  // more dups in B
        conflict_prob: 0.30,             // high chance of conflicting values for same key
        long_field_prob: 0.001,
        unicode: true,
        crlf: false,
        seed: 1337,
    }).unwrap();

    let a = f1.to_string_lossy().to_string();
    let b = f2.to_string_lossy().to_string();
    let files: Vec<&String> = vec![&a, &b];

    // UNION: size should be ~ rows + rows - overlap
    let union = perform_union(files.clone()).unwrap();
    let expected_union_len = (rows as f64 + rows as f64 - (rows as f64 * 0.5)).round() as usize;
    assert!(union.len() >= expected_union_len - 5_000 && union.len() <= expected_union_len + 5_000,
            "union size {}, expected around {}", union.len(), expected_union_len);

    // INTERSECT: ≈ overlap (50%)
    let inter = perform_intersect(files.clone()).unwrap();
    let expected_inter_len = (rows as f64 * 0.5).round() as usize;
    assert!(inter.len() >= expected_inter_len - 5_000 && inter.len() <= expected_inter_len + 5_000,
            "intersect size {}, expected around {}", inter.len(), expected_inter_len);

    // DIFFA (A \ B): roughly half
    let diffa = perform_diffa(files.clone()).unwrap();
    let expected_diffa_len = (rows as f64 * 0.5).round() as usize;
    assert!(diffa.len() >= expected_diffa_len - 5_000 && diffa.len() <= expected_diffa_len + 5_000,
            "diff size {}, expected around {}", diffa.len(), expected_diffa_len);

    // XOR: keys present in exactly one file ≈ rows (50%+50%)
    // NOTE: your XOR currently keeps the last value seen and filters odd-count keys.
    let xor = perform_xor(files.clone()).unwrap();
    let expected_xor_len = expected_union_len - expected_inter_len; // same as symmetric diff estimate
    assert!(xor.len() >= expected_xor_len - 10_000 && xor.len() <= expected_xor_len + 10_000,
            "xor size {}, expected around {}", xor.len(), expected_xor_len);
}

/// A truly huge file test — disabled by default.
/// Run explicitly: `cargo test --release -- huge_1m -- --ignored --nocapture`
#[ignore]
#[test]
fn huge_1m() {
    let rows = 1_000_000u64;
    let dir = tempdir().unwrap();
    let f1 = dir.path().join("big_a.csv");
    let f2 = dir.path().join("big_b.csv");

    write_csv(&f1, GenCfg { rows, key_start: 0,  dup_prob: 0.01, conflict_prob: 0.05, long_field_prob: 0.0, unicode: false, crlf: false, seed: 777 }).unwrap();
    write_csv(&f2, GenCfg { rows, key_start: rows/2, dup_prob: 0.01, conflict_prob: 0.05, long_field_prob: 0.0, unicode: false, crlf: false, seed: 778 }).unwrap();

    let a = f1.to_string_lossy().to_string();
    let b = f2.to_string_lossy().to_string();
    let files: Vec<&String> = vec![&a, &b];

    let union = perform_union(files.clone()).unwrap();
    assert!(union.len() > 1_500_000 / 2); // sanity

    let inter = perform_intersect(files.clone()).unwrap();
    assert!(inter.len() > 400_000); // sanity

    // Just ensure no panics / OOM and reasonable sizes
}

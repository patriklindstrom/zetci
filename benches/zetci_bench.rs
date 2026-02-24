use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use tempfile::tempdir;
use zetci::set_operations::{union::perform_union, intersect::perform_intersect, diffa::perform_diffa, xor::perform_xor};


#[path = "../tests/support/mod.rs"]
mod tests_support;

use tests_support::gen::{write_csv, GenCfg};


fn synth_pair(rows: u64) -> (tempfile::TempDir, String, String) {
    let dir = tempdir().unwrap();
    let f1 = dir.path().join("a.csv");
    let f2 = dir.path().join("b.csv");

    write_csv(&f1, GenCfg{
        rows, key_start: 0, dup_prob: 0.0, conflict_prob: 0.05, long_field_prob: 0.0, unicode: false, crlf: false, seed: 1
    }).unwrap();
    write_csv(&f2, GenCfg{
        rows, key_start: rows/2, dup_prob: 0.0, conflict_prob: 0.05, long_field_prob: 0.0, unicode: false, crlf: false, seed: 2
    }).unwrap();

    (dir, f1.to_string_lossy().to_string(), f2.to_string_lossy().to_string())
}

fn bench_ops(c: &mut Criterion) {
    let mut g = c.benchmark_group("zetci_set_ops");
    for &rows in &[50_000u64, 200_000u64] {
        g.bench_function(format!("union_{}", rows), |b| {
            b.iter_batched(
                || {
                    let (dir, a, b) = synth_pair(rows);
                    (dir, vec![a, b])
                },
                |(dir, files)| {
                    let refs: Vec<&String> = files.iter().collect();
                    let _ = perform_union(refs).unwrap();
                },
                BatchSize::LargeInput,
            )
        });

        g.bench_function(format!("intersect_{}", rows), |b| {
            b.iter_batched(
                || {
                    let (dir, a, b) = synth_pair(rows);
                    (dir, vec![a, b])
                },
                |(dir, files)| {
                    let refs: Vec<&String> = files.iter().collect();
                    let _ = perform_intersect(refs).unwrap();
                },
                BatchSize::LargeInput,
            )
        });

        g.bench_function(format!("diffa_{}", rows), |b| {
            b.iter_batched(
                || {
                    let (dir, a, b) = synth_pair(rows);
                    (dir, vec![a, b])
                },
                |(dir, files)| {
                    let refs: Vec<&String> = files.iter().collect();
                    let _ = perform_diffa(refs).unwrap();
                },
                BatchSize::LargeInput,
            )
        });

        g.bench_function(format!("xor_{}", rows), |b| {
            b.iter_batched(
                || {
                    let (dir, a, b) = synth_pair(rows);
                    (dir, vec![a, b])
                },
                |(dir, files)| {
                    let refs: Vec<&String> = files.iter().collect();
                    let _ = perform_xor(refs, "first").unwrap();
                },
                BatchSize::LargeInput,
            )
        });
    } // Close the for loop here
    g.finish(); // Finish the benchmark group
}
criterion_group!(benches, bench_ops);
criterion_main!(benches);

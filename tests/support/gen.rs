use rand::{rngs::StdRng, Rng, SeedableRng, RngExt}; // Add RngExt here
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Controls how rows are synthesized
#[derive(Clone, Debug)]
pub struct GenCfg {
    /// Number of rows to write
    pub rows: u64,
    /// Start key id (so multiple files can interleave/overlap deterministically)
    pub key_start: u64,
    /// Probability [0.0..1.0] that the generated key will be a duplicate of a recent key (intra-file dup)
    pub dup_prob: f32,
    /// Probability [0.0..1.0] that we mutate value fields (to create cross-file conflicts on same key)
    pub conflict_prob: f32,
    /// Probability a field becomes very long (tests CSV/IO performance)
    pub long_field_prob: f32,
    /// Include some non-ASCII unicode?
    pub unicode: bool,
    /// Force CRLF newlines (Windows-style) if true; else LF
    pub crlf: bool,
    /// Deterministic RNG seed
    pub seed: u64,
}

/// Returns a "person-like" row as Vec<String>, always **6 columns** (to match your current format):
/// [id, Firstname, Lastname, Distance, Time, ShoeBrand]
fn synth_row(
    mut rng: &mut StdRng,
    id: u64,
    conflict: bool,
    unicode: bool,
    long_field_prob: f32,
) -> Vec<String> {
    // Some small vocabularies
    const FIRST: &[&str] = &["Adam","Bertil","Cecilia","Emma","Anna","Hampus","Olle","Lilith","Zorro","Jane","Joanna","Björn","Åsa","Örjan","Élodie","Mårten"];
    const LAST:  &[&str] = &["Svensson","Olsson","Lindström","Larsson","Lindstrom","Collie","Karlsson","Andersson","Lindqvist","Åkesson","Öberg","Björn"];
    const BRAND: &[&str] = &["Nike","Adidas","Bare","Na","Tass","Sandal","Puma","Asics"];
    const DIST:  &[&str] = &["500 m","1000 m","2000 m","3000 m","6000 m"];

    // Use a helper function instead of a closure to avoid borrow issues
    fn choose(rng: &mut StdRng, arr: &[&str]) -> String {
        arr[rng.random_range(0..arr.len())].to_string()
    }

    let mut first = choose(rng, FIRST);
    let mut last  = choose(rng, LAST);
    let mut dist  = choose(rng, DIST);
    let mut time  = format!("{} min {} s", rng.random_range(3..30), rng.random_range(0..60));
    let mut brand = choose(rng, BRAND);

    // occasional long fields - use a helper function instead of closure
    fn maybe_make_long(rng: &mut StdRng, s: &mut String, label: &str, long_field_prob: f32) {
        if rng.random::<f32>() < long_field_prob {
            s.push_str(&"X".repeat(8_000)); // ~8 KB field to push CSV/IO
            s.push_str(label);
        }
    }

    maybe_make_long(rng, &mut first, "F", long_field_prob);
    maybe_make_long(rng, &mut last,  "L", long_field_prob);
    maybe_make_long(rng, &mut brand, "B", long_field_prob);

    // unicode sprinkle
    if unicode {
        let unicodes = ["Å", "Ä", "Ö", "é", "ß", "—", "猫", "🐟", "🙂"];
        if rng.random::<bool>() { first.push_str(unicodes[rng.random_range(0..unicodes.len())]); }
        if rng.random::<bool>() { last.push_str(unicodes[rng.random_range(0..unicodes.len())]); }
    }

    // value conflict knob: if true, nudge fields so same key can differ across files
    if conflict {
        if rng.random::<bool>() { brand.push_str("-X"); }
        if rng.random::<bool>() { dist = format!("{} m", 500 * rng.random_range(1..16)); }
        if rng.random::<bool>() { time = format!("{} min {} s", rng.random_range(3..40), rng.random_range(0..60)); }
    }

    vec![
        id.to_string(),
        first, last, dist, time, brand
    ]
}

/// Writes a headerless CSV with **rows** synthetic records, using streaming IO.
/// Keys are mostly sequential from key_start except for occasional in-file dups (dup_prob).
/// conflict_prob controls the chance that a row’s *values* differ (useful when generating the
/// *other* file with the same keys to force conflicting values).
pub fn write_csv<P: AsRef<Path>>(path: P, cfg: GenCfg) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut w = BufWriter::new(file);
    let mut rng = StdRng::seed_from_u64(cfg.seed);

    // for duplicate generation we will keep a small ring buffer of recent keys
    const RING: usize = 1024;
    let mut recent: Vec<u64> = Vec::with_capacity(RING);

    for i in 0..cfg.rows {
        let mut key = cfg.key_start + i;

        if !recent.is_empty() && rng.random::<f32>() < cfg.dup_prob {
            key = *recent.get(rng.random_range(0..recent.len())).unwrap();
        }

        // Evaluate conflict probability BEFORE calling synth_row to avoid simultaneous mutable borrows
        let is_conflict = rng.random::<f32>() < cfg.conflict_prob;
        let row = synth_row(&mut rng, key, is_conflict, cfg.unicode, cfg.long_field_prob);
        // NB: values can contain commas/Unicode—csv crate in *reader* handles quoting.
        // To keep generation simple and fast, we won't add quotes here; fields themselves avoid commas.

        let line = row.join(",");
        if cfg.crlf {
            w.write_all(line.as_bytes())?;
            w.write_all(b"\r\n")?;
        } else {
            w.write_all(line.as_bytes())?;
            w.write_all(b"\n")?;
        }

        // update ring buffer
        if recent.len() < RING { recent.push(key); } else {
            let idx = i as usize % RING;
            recent[idx] = key;
        }
    }
    w.flush()?;
    Ok(())
}

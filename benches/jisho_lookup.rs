use criterion::{criterion_group, criterion_main, Criterion};
use jisho::{lookup, Entry};

fn entry() -> Entry {
    Entry {
        kanji: "緑".to_string(),
        reading: "みどり".to_string(),
        meanings: vec![
            "green".to_string(),
            "greenery".to_string(),
            "verdure".to_string(),
        ],
        frequency: 3,
    }
}

fn kanji_lookup() {
    let results = lookup("緑");
    assert_eq!(results.first().unwrap(), &&entry())
}

fn reading_lookup() {
    let results = lookup("みどり");
    assert_eq!(results.first().unwrap(), &&entry())
}

fn meaning_lookup() {
    let results = lookup("green");
    assert!(results.contains(&&entry()))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("kanji_lookup", |b| b.iter(|| kanji_lookup()));
    c.bench_function("reading_lookup", |b| b.iter(|| reading_lookup()));
    c.bench_function("meaning_lookup", |b| b.iter(|| meaning_lookup()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

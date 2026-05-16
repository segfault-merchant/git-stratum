use std::str::FromStr;

use criterion::{Criterion, criterion_group, criterion_main};

use stratum::Actor;

fn bench_from_str(c: &mut Criterion) {
    c.bench_function("ACTOR FROM STR", |b| {
        b.iter(|| Actor::from_str("name <email>"))
    });
}

criterion_group!(benches, bench_from_str);
criterion_main!(benches);

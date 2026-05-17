use criterion::{Criterion, criterion_group, criterion_main};

mod common;
use common::repo_fixture;

fn bench_stats(c: &mut Criterion) {
    repo_fixture("diff", |repo| {
        let commit = repo.head().expect("Failed to get head");

        c.bench_function("STAT GENERATION", |b| b.iter(|| commit.insertions()));
    });
}

fn bench_mfiles(c: &mut Criterion) {
    repo_fixture("diff", |repo| {
        let commit = repo.head().expect("Failed to get head");

        c.bench_function("MFILE GENERATION", |b| b.iter(|| commit.mod_files()));
    });
}

criterion_group!(stat_benches, bench_stats);
criterion_group!(mfile_benches, bench_mfiles);
criterion_main!(stat_benches, mfile_benches);

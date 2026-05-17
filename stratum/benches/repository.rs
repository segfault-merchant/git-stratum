use criterion::{Criterion, criterion_group, criterion_main};

mod common;
use common::repo_fixture;

fn bench_traversal(c: &mut Criterion) {
    repo_fixture("complex_repo", |repo| {
        c.bench_function("COMMIT TRAVERSAL", |b| b.iter(|| repo.traverse_commits()));
    });
}

criterion_group!(benches, bench_traversal);
criterion_main!(benches);

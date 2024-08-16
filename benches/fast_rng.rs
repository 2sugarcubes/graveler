use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graveler::check_n_games;

pub fn check_games(c: &mut Criterion) {
    c.bench_function("check 1m", |b| {
        b.iter(|| check_n_games(black_box(1_000_000)))
    });
}

criterion_group!(benches, check_games);
criterion_main!(benches);

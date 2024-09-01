use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graveler::{check_from_half_games, check_n_games, QuickerRng};

pub fn check_games(c: &mut Criterion) {
    c.bench_function("check 1b (threaded estimate)", |b| {
        b.iter(|| check_n_games(1_000_000_000 / 12 + 1))
    });
    c.bench_function("check 1m", |b| b.iter(|| check_n_games(1_000_000)));
    c.bench_function("check 1b cheaky (threaded estimate)", |b| {
        b.iter(|| check_from_half_games())
    });
    c.bench_function("gen 1m numbers", |b| {
        b.iter(|| gen_numbers(black_box(1_000_000)))
    });
}

fn gen_numbers(n: u64) {
    let mut rng = QuickerRng {
        state_1: 0x5555_5555_5555_5555,
        state_2: 0xF0F0_F0F0_F0F0_F0F0,
    };
    for _ in 0..n {
        rng.next_state()
    }
}

criterion_group!(benches, check_games);
criterion_main!(benches);

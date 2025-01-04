use cached::proc_macro::cached;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};

const NUMBER_OF_TRIALS: usize = 231;

pub fn bernoulli_game_bench(c: &mut Criterion) {
    c.bench_function("check 1m with bernoulli", |b| {
        b.iter(|| bernoulli_games(1_000_000))
    });

    c.bench_function("Check 1b with regular", |b| {
        b.iter(|| graveler::check_n_games(1_000_000))
    });
}

criterion_group!(benches, bernoulli_game_bench);
criterion_main!(benches);
fn generate_lookup_table() -> [u128; NUMBER_OF_TRIALS + 1] {
    // -(log2(0.25))
    const P: u128 = 0x2;
    // little endian fixed point representation of 0.75
    const Q: u128 = 0b11;

    // 0.75 * 0.25 = 0.1875 (0.125 + 0.0625)
    // 0b11 << 2 = 0b0011 (0b001 + 0b0001)
    //
    // 0.75 ^ 2 = 0.5625 = (0.5 + 0.0625) = (0b1001)

    let mut res = [0; NUMBER_OF_TRIALS + 1];

    for k in 0..=NUMBER_OF_TRIALS {
        let binom = n_c_k(NUMBER_OF_TRIALS as u8, k as u8);
        // Equivalent to Q^k-n * p^k, and returns the big endian fixed point representation of the
        // probablity
        let probability = (Q.pow((NUMBER_OF_TRIALS - k) as u32) << (P << k)).reverse_bits();

        res[k] = binom.wrapping_mul(probability);
    }

    let mut result = [res[0]; NUMBER_OF_TRIALS + 1];

    // Transform into a cumulative distribution function
    for i in 1..res.len() {
        result[i] = result[i - 1] + res[i];
    }

    return result;
}

#[cached]
fn n_c_k(n: u8, k: u8) -> u128 {
    if k == 0 || k == n + 1 {
        return 1;
    } else if n == 1 || n == 0 {
        return 1;
    }
    n_c_k(n - 1, k - 1) + n_c_k(n - 1, k)
}

fn bernoulli_game(id: u128, lookup_table: [u128; NUMBER_OF_TRIALS + 1]) -> usize {
    for k in 0..lookup_table.len() {
        if id > (lookup_table[k]) {
            return k - 1;
        }
    }

    return lookup_table
        .binary_search(&id)
        .and_then(|x| Ok(x + 1))
        .unwrap_or_else(|x| x);
}

fn bernoulli_games(number_of_games: usize) -> usize {
    let lookup_table = generate_lookup_table();

    let mut rng = thread_rng();
    let mut rng = graveler::QuickerRng {
        state_1: rng.gen(),
        state_2: rng.gen(),
    };

    let mut max = 0;
    for _ in 0..number_of_games / 2 {
        max = max.max(bernoulli_game(
            (rng.state_1 as u128) << 64 | rng.state_2 as u128,
            lookup_table,
        ));
        max = max.max(bernoulli_game(
            (rng.state_2 as u128) << 64 | rng.state_1 as u128,
            lookup_table,
        ));
        rng.next_state();
    }

    return max;
}

use graveler::check_n_games;
use rayon::prelude::*;

fn main() {
    const ONE_BILLION_ROWS: u64 = 1_000_000_000;
    let step_size = ONE_BILLION_ROWS / 12 + 1;

    let max_ones = (0_usize..12)
        .into_par_iter()
        .map(|_| check_n_games(step_size))
        .max()
        .unwrap_or(0_u8);

    println!("Highest Ones Roll: {}", max_ones);
    println!("Number of Roll Sessions: {}", step_size * 12);
}

use graveler::check_n_games;
use rayon::prelude::*;

fn main() {
    const THREADS: u64 = 12;
    const ONE_BILLION_ROWS: u64 = 1_000_000_000;
    const STEP_SIZE: u64 = ONE_BILLION_ROWS / THREADS + 1;

    // Generate 12 threads
    let max_ones = (0..THREADS)
        .into_par_iter()
        // For each of them play games equal to 1,000,000,000/theads + 1 (to make sure we play AT
        // LEAST one billion games)
        .map(|_| check_n_games(STEP_SIZE))
        // Find the maximum of the results
        .max()
        // Return 0 if all threads paniced
        .unwrap_or(0);

    // Print statistics like in the video
    println!("Highest Ones Roll: {}", max_ones);
    println!("Number of Roll Sessions: {}", STEP_SIZE * 12);
}

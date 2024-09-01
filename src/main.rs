use graveler::check_from_half_games;
use rayon::prelude::*;

fn main() {
    const THREADS: u64 = 12;
    const STEP_SIZE: u64 = 83340505; // binom(12911, 2)

    // Generate 12 threads
    let results = (0..THREADS)
        .into_par_iter()
        // For each of them play games equal to 1,000,000,000/theads + 1 (to make sure we play AT
        // LEAST one billion games)
        .map(|_| check_from_half_games())
        // Find the maximum of the results
        .max()
        // Return 0 if all threads paniced
        .unwrap_or(0);

    // Print statistics like in the video
    println!("Highest Ones Roll: {}", results);
    println!("Number of Roll Sessions: {}", STEP_SIZE * THREADS);
}

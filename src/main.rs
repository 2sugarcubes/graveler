use graveler::check_from_half_games;
use rayon::prelude::*;

fn main() {
    const THREADS: u64 = 12;

    // Generate 12 threads
    let results = (0..THREADS)
        .into_par_iter()
        // For each of them play games equal to 1,000,000,000/theads + 1 (to make sure we play AT
        // LEAST one billion games)
        .map(|_| check_from_half_games())
        // Find the maximum of the results
        .max_by_key(|a| a.0)
        // Return 0 if all threads paniced
        .unwrap_or((0, 0));

    // Print statistics like in the video
    println!("Highest Ones Roll: {}", results.0);
    println!("Number of Roll Sessions: {}", results.1 * THREADS);
}

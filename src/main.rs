use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut ones = 0;
    let mut max_ones = 0;
    const ONE_MILLION_DOLLARS: u64 = 1_000_000;
    for _ in 0..ONE_MILLION_DOLLARS {
        for _ in 0..231 {
            let rand_number: u8 = rng.gen();
            if rand_number % 4 == 0 {
                ones += 1;
            }
        }
        if ones > max_ones {
            max_ones = ones;
        }
        ones = 0;
    }

    println!("Highest Ones Roll: {}", max_ones);
    println!("Number of Roll Sessions: {}", ONE_MILLION_DOLLARS);
}

use rand::{rngs::ThreadRng, Rng};
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

fn check_n_games(n: u64) -> u8 {
    let rng = rand::thread_rng();
    let mut ones = 0_u8;
    let mut max_ones = 0_u8;
    let mut quicker_rng = QuickerRng::new(rng);

    // Not checking for if we got enough ones because it is too costly for such a slim chance
    for _ in 0..n {
        // Since we are now rolling 64 random numbers at a time we can only use 3 random numbers
        // before we do something special.
        for _ in 0..3 {
            // By anding two random numbers we have a 25% chance of having a 1 for any bit,
            // therefore the count of ones is the number of "ones rolled" on a four sided dice.
            let state = quicker_rng.get_chances();
            quicker_rng.next_state();
            ones += state.count_ones() as u8;
        }

        // Last one is special because we only check 39 bits (That's what the last & is for)
        let state = quicker_rng.get_chances() & 0x7F_FF_FF_FF_FF;
        ones += state.count_ones() as u8;

        //max_ones = max_ones.max(ones);
        if ones > max_ones {
            max_ones = ones;
        }
        //max_ones = unsafe {
        //   std::mem::transmute::<bool, u8>(ones < max_ones) * max_ones
        //    + std::mem::transmute::<bool, u8>(ones >= max_ones) * ones
        //};
        ones = 0;
    }
    return max_ones;
}

struct QuickerRng {
    state_1: u64,
    state_2: u64,
}

impl QuickerRng {
    fn new(mut rng: ThreadRng) -> Self {
        Self {
            state_1: rng.gen(),
            state_2: rng.gen(),
        }
    }

    fn next_state(&mut self) {
        self.state_1 ^= self.state_1 << 7;
        self.state_2 ^= self.state_2 << 7;

        self.state_1 ^= self.state_1 >> 9;
        self.state_2 ^= self.state_2 >> 9;
    }

    // TODO remove alocate here
    fn get_chances(&self) -> u64 {
        self.state_1 & self.state_2
    }
}

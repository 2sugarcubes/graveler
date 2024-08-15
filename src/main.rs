use rand::Rng;
use rayon::prelude::*;

fn main() {
    const ONE_BILLION_ROWS: u64 = 1_000_000_000;
    let step_size = ONE_BILLION_ROWS / 8;

    let max_ones = (0_usize..8)
        .into_par_iter()
        .map(|_| check_n_games(step_size))
        .max()
        .unwrap_or(0_u32);

    println!("Highest Ones Roll: {}", max_ones);
    println!("Number of Roll Sessions: {}", step_size * 8);
}

fn check_n_games(n: u64) -> u32 {
    let mut rng = rand::thread_rng();
    let mut ones = 0;
    let mut max_ones = 0;
    let mut rng1 = QuickRng { state: rng.gen() };
    let mut rng2 = QuickRng { state: rng.gen() };

    // Not checking for if we got enough ones because it is too costly for such a slim chance
    for _ in 0..n {
        // Since we are now rolling 64 random numbers at a time we can only use 3 random numbers
        // before we do something special.
        for _ in 0..3 {
            // By anding two random numbers we have a 25% chance of having a 1 for any bit,
            // therefore the count of ones is the number of "ones rolled" on a four sided dice.
            let state = rng1.get_state() & rng2.get_state();
            ones += state.count_ones();
        }

        // Last one is special because we only check 39 bits (That's what the last & is for)
        let state = rng1.get_state() & rng2.get_state() & 0x7F_FF_FF_FF_FF;
        ones += state.count_ones();

        if ones > max_ones {
            max_ones = ones;
        }
        ones = 0;
    }
    return max_ones;
}

struct QuickRng {
    state: u64,
}

impl QuickRng {
    pub fn next_state(&mut self) {
        self.state ^= self.state << 7;
        self.state ^= self.state >> 9;
    }

    pub fn get_state(&mut self) -> u64 {
        let result = self.state;
        self.next_state();
        return result;
    }
}

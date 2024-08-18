use rand::{rngs::ThreadRng, Rng};

pub fn check_n_games(n: u64) -> u32 {
    // assign memory out of the hot path.

    // This will generate initial states for our PRNG
    let rng = rand::thread_rng();
    // This will store how many ones were found in a round
    let mut ones;
    // This will store the maximum number of ones that was found
    let mut max_ones = 0;
    // Here is our quick XorShift RNG that will be used in the hot path
    let mut quicker_rng = QuickerRng::new(rng);

    // Not checking for if we got enough ones because it is too costly for such a slim chance
    for _ in 0..n {
        // Since we are rolling 64 random numbers at a time we can only use 3 random numbers
        // before we need to do something special.
        ones = quicker_rng.get_chances().count_ones();
        ones += quicker_rng.get_chances().count_ones();
        ones += quicker_rng.get_chances().count_ones();

        // Last one is special because we only check 39 bits (That's what the last & is for)
        // because we have already checked `64*3=192` games.
        ones += (quicker_rng.get_chances() & 0x7F_FF_FF_FF_FF).count_ones();

        if ones > max_ones {
            max_ones = ones;
        }
    }
    return max_ones;
}

pub struct QuickerRng {
    pub state_1: u64,
    pub state_2: u64,
}

/// XorShift PRNG based off https://en.wikipedia.org/wiki/Xorshift#Example_implementation
impl QuickerRng {
    /// Generate a PRNG from a random state, destroying the random state in the process
    fn new(mut rng: ThreadRng) -> Self {
        Self {
            state_1: rng.gen(),
            state_2: rng.gen(),
        }
    }

    // Lifted from [here](https://en.wikipedia.org/wiki/Xorshift#Example_implementation), but
    // modified slightly to take advantage of SIMD instructions (Basically being able to generate
    // two random numbers for the cost of one)
    pub fn next_state(&mut self) {
        self.state_1 ^= self.state_1 << 7;
        self.state_2 ^= self.state_2 << 7;

        self.state_1 ^= self.state_1 >> 9;
        self.state_2 ^= self.state_2 >> 9;
    }

    /// Uses the two random states to generate a number with a 25% chance of having an 1 in each
    /// bit.
    pub fn get_chances(&mut self) -> u64 {
        self.next_state();
        self.state_1 & self.state_2
    }
}

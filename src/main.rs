use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut ones = 0;
    let mut max_ones = 0;
    const ONE_MILLION_DOLLARS: u64 = 1_000_000;

    let mut quick_rng = QuickRng {
        pointer: 0,
        state: rng.gen(),
    };

    // Not checking for if we got enough ones because it is too costly for such a slim chance
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

struct QuickRng {
    state: u64,
    pointer: u8,
}

impl QuickRng {
    pub fn next_state(&mut self) {
        self.state ^= self.state << 7;
        self.state ^= self.state >> 9;
    }

    pub fn next_digit(&mut self) -> u8 {
        // Get the next unread crumb
        let result = self.state & (0b11 << self.pointer) >> self.pointer;
        if self.pointer >= 126 {
            self.pointer = 0;
            self.next_state();
        } else {
            self.pointer += 2;
        }
        return result as u8;
    }
}

#[cfg(test)]
mod test {
    use crate::QuickRng;

    #[test]
    fn randomness() {
        let mut rng = QuickRng {
            // Random state from from https://www.random.org/integers/?num=4&min=1&max=65536&col=8&base=16&format=plain&rnd=date.2024-08-15
            state: 0xc7e5_9803_cf93_015d,
            pointer: 0,
        };
        let trials = (2 as u32).pow(32);
        let success_probability = 0.25;

        let mut counts = [0, 0, 0, 0];
        for _ in 0..trials {
            counts[rng.next_digit() as usize] += 1;
        }

        let mean = success_probability * trials as f32;
        let mut deviation = mean * (1.0 - success_probability);
        deviation = deviation.sqrt();
        for i in counts {
            println!("{}", i);

            // 68% chance of falling within one standard deviations of the mean
            assert!(mean - deviation <= i as f32 && i as f32 <= mean + deviation);
        }
        // probability they are all in that range by chance = .68^4 or 16%
    }
}

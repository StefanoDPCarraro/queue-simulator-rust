const A:u128 = 6364136223846793005;
const C:u128 = 1442695040888963407;
const M:u128 = u64::MAX as u128;

static mut current_seed:u128 = 7;

const REPETICOES:u16 = 1000;

fn main() {
    for _ in 0..REPETICOES {
        let next = next_random();
        println!("{}", next);
    }
}


// Given a number, calculates the next one in the sequence
// using it in a loop for M times gives all the numbers
// before they start to repeat
fn linear_congruencial_gen(seed: u128) -> u128 {
    (A * seed + C) % M  as u128
}

// Returns the next random number given a global
// seed
fn next_random() -> f64 {
    unsafe { current_seed = linear_congruencial_gen(current_seed) };
    (unsafe { current_seed } as f64)/(M as f64)
}
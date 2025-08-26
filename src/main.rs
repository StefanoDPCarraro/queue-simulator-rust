// Vars for RNG
const A:u128 = 6364136223846793005;
const C:u128 = 1442695040888963407;
const M:u128 = u64::MAX as u128;

const REPETICOES:u16 = 1000;

static mut current_seed:u128 = 7;

// Vars for the queue
const CHEGADA_MIN:u16 = 1;
const CHEGADA_MAX:u16 = 5;
const ATENDIMENTO_MIN:u16 = 2;
const ATENDIMENTO_MAX:u16 = 6;

const SERVIDORES:u16 = 3;
const CAPACIDADE:u16 = 5;

// Global vars for queue state
static mut tamanho_atual:u16 = 0;

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

fn handle_chegada() -> bool {
    unsafe {
        if tamanho_atual < CAPACIDADE {
            tamanho_atual += 1;
            return true;
        }
    }
    false
}

fn handle_saida() -> bool {
    unsafe {
        if tamanho_atual > 0 {
            tamanho_atual -= 1;
            return true;
        }
    }
    false
}

fn schedule_chegada() -> u16 {
    let random = next_random();
    (CHEGADA_MIN as f64 + (CHEGADA_MAX - CHEGADA_MIN) as f64 * random) as u16
}

fn schedule_saida() -> u16 {
    let random = next_random();
    (ATENDIMENTO_MIN as f64 + (ATENDIMENTO_MAX - ATENDIMENTO_MIN) as f64 * random) as u16
}
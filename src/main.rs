use std::collections::LinkedList;

// Vars for RNG
const A:u128 = 6364136223846793005;
const C:u128 = 1442695040888963407;
const M:u128 = u64::MAX as u128;

const REPETICOES:u16 = 1000;

static mut CURRENT_SEED:u128 = 7;

// Vars for the queue
const CHEGADA_MIN:u16 = 1;
const CHEGADA_MAX:u16 = 5;
const ATENDIMENTO_MIN:u16 = 2;
const ATENDIMENTO_MAX:u16 = 6;

const SERVIDORES:u16 = 3;
const CAPACIDADE:u16 = 5;

// Global vars for queue state
static mut TAMANHO_ATUAL:u16 = 0;
static mut TEMPO_GLOBAL: f64 = 0.0;
static mut ESCALANADOR: LinkedList<f64> = LinkedList::new();
static mut LOSS: u16 = 0;


fn main() {
    for _ in 0..REPETICOES {
        let next = next_random();
        println!("{}", next);
    }
}

#[allow(static_mut_refs)]
fn acumula_tempo() {
    unsafe {
        if TAMANHO_ATUAL < CAPACIDADE {
            TAMANHO_ATUAL += 1;

            if TAMANHO_ATUAL <= SERVIDORES {
                ESCALANADOR.push_back(TEMPO_GLOBAL + schedule_saida());
            }
        }
        LOSS += 1;
        ESCALANADOR.push_back(TEMPO_GLOBAL + schedule_chegada());
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
    unsafe { CURRENT_SEED = linear_congruencial_gen(CURRENT_SEED) };
    (unsafe { CURRENT_SEED } as f64)/(M as f64)
}

fn handle_chegada() -> bool {
    unsafe {
        if TAMANHO_ATUAL < CAPACIDADE {
            return true;
        }
    }
    false
}

fn handle_saida() -> bool {
    unsafe {
        if TAMANHO_ATUAL > 0 {
            return true;
        }
    }
    false
}

fn schedule_chegada() -> f64 {
    let random = next_random();
    (CHEGADA_MIN as f64 + (CHEGADA_MAX - CHEGADA_MIN) as f64 * random) as f64
}

fn schedule_saida() -> f64 {
    let random = next_random();
    (ATENDIMENTO_MIN as f64 + (ATENDIMENTO_MAX - ATENDIMENTO_MIN) as f64 * random) as f64
}
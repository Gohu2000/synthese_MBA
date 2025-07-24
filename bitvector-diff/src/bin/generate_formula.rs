use std::{fs::File, io::Write};

use bitvector_diff::{formula::Node};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

fn usize_to_string(n: usize) -> String {
    if n < 10 {
        format!("0{n}")
    }
    else {
        format!("{n}")
    }
}

fn generate_formula(n: usize, folder: &str) {
    let mut rng: ChaCha20Rng = SeedableRng::seed_from_u64(42);
    for size in 1..51 {
        for n_inputs in 1..31 {
            let mut data_file = File::create(format!("{folder}/formula_{}_{}.txt", usize_to_string(n_inputs), usize_to_string(size))).expect("creation failed");
            let mut buffer = String::new();
            for _ in 0..n {
                let formula = Node::random(n_inputs, size, 1, &mut rng, true);
                buffer.push_str(&formula.to_string());
                buffer.push('\n');
            }
            data_file.write(buffer.as_bytes()).expect("write failed");
        }
    }
}

fn main() {
    generate_formula(50, "examples_formula")
}
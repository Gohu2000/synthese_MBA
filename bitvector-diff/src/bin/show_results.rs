use std::{fs::File, io::{Read, Write},};

use bitvector_diff::solving::results::{FinalResult, FoundResult};
use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    /// Name of the .json file to read.
    input_filename: String,
}

fn main() {
    let CliArgs { input_filename } = CliArgs::parse();
    let mut data_file = File::open(input_filename).unwrap();
    let mut file_content = String::new();
    data_file.read_to_string(&mut file_content).unwrap();
    let mut compteur_found = 0;
    let mut compteur_equivalence = 0;
    for line in file_content.lines() {
        if let FinalResult::Found(FoundResult { formula, time, solution, size, equivalence, solver, param, instance }) = FinalResult::from_str(line) {
            compteur_found += 1;
            if equivalence > 0.999 {
                compteur_equivalence += 1;
            }
            println!("{formula} {time}ms {equivalence}")
        }
    }
    println!("Nombre de formules trouvées : {compteur_found}");
    println!("Nombre de formules équivalentes trouvées : {compteur_equivalence}");
}
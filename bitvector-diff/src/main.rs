use bitvector_diff::solving::{Benchmark, Solver};
use rand::rng;


fn main() {
    let params = (5, 32, 15); // (n_inputs, n_examples, size)
    let mut rng = rng();
    let benchmark = Benchmark::new(params, &mut rng, 100);
    benchmark.to_file("benchmark.txt");
    let benchmark = Benchmark::from_file("benchmark.txt");
    let solver = Solver::Algonaif;
    benchmark.solve_print(solver, &mut rng, params)
}
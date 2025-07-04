use bitvector_diff::solving::{Benchmark, Solver, Greedy, Enumerator, Parametres};
use rand::rng;


fn main() {
    let params: Parametres = (5, 512, 10); // (n_inputs, n_examples, size)
    let mut rng = rng();
    //let benchmark = Benchmark::new(params, &mut rng, 100);
    //benchmark.to_file("benchmark.txt");
    let benchmark = Benchmark::from_file("benchmark.txt");
    let greedy = Greedy::Naif(50.);
    let enumerator = Enumerator::ProgressiveSize(20, 100, 100);
    let solver = Solver{greedy, enumerator};
    benchmark.solve_print(solver, &mut rng, params)
}
use bitvector_diff::solving::{Selection, Solver, Greedy, Enumerator, Parametres};
use rand::rng;

fn main() {
    let params: Parametres = (5, 128, 10); // (n_inputs, n_examples, size)
    let mut rng = rng();
    let selection = Selection::new(params, &mut rng, 50, "selection.txt");
    //let selection = Selection::from_file("selection.txt");
    let greedy = Greedy::Naif(50.);
    let enumerator = Enumerator::ProgressiveSize(20, 10, 100);
    let solver = Solver{greedy, enumerator};
    selection.solve_print(solver, &mut rng, params, true);
    for _ in 0..20 {
        selection.solve_print(solver, &mut rng, params, false);
    }
}
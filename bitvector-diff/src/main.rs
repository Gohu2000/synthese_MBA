use bitvector_diff::solving::{
    results::Interpretor, Enumerator, Greedy, JsonData, Parametres, Selection, Solver
};
use clap::Parser;
use rand::{rng, Rng};

#[derive(Parser)]
struct CliArgs {
    /// Name of the .json file to read.
    input_filename: String,
}

fn old_calcul(selection: Selection, params:Parametres, rng: &mut impl Rng) {
    //let mut selection = Selection::new(params, &mut rng, 50, "selection.txt");
    let params: Parametres = (5, 128, 10);
    let selection = Selection::from_file("selection.txt");
    let greedy = Greedy::Naif(50.);
    let enumerator = Enumerator::ProgressiveSize(20, 10, 100);
    let solver = Solver{greedy, enumerator};
    selection.solve_print(solver, rng, params, true);
    for _ in 0..20 {
        //selection.change_instances(&mut rng);
        selection.solve_print(solver, rng, params, false);
    }
}

fn main() {
    let CliArgs { input_filename } = CliArgs::parse();
    let mut rng = rng();
    let greedy = Greedy::Naif(50.);
    let enumerator = Enumerator::ProgressiveSize(20, 100, 300);
    let solver = Solver{greedy, enumerator};
    let json_data = JsonData::from_file(&input_filename);
    json_data.solve_print(solver, &mut rng);
    //let interpretor = Interpretor::from_file("interpretor.txt");
    //interpretor.print_if_counter(selection.solutions, 0);
}
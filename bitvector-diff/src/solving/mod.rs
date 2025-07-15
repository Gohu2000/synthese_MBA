use crate::{formula::{
    grad::Scores, Node
}, solving::results::Interpretor};
use rand::{rng, Rng};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::File,
    io::{Write, Read},
    time::Instant,
};
use conv::ValueFrom;

pub mod results;

/// (n_inputs, n_examples, size)
pub type Parametres = (usize, usize, usize);

#[derive(Deserialize, Serialize)]
pub struct JsonData {
    instance: Instance,
    solution: String,
    param: Parametres
}

#[derive(Deserialize, Serialize)]
pub struct Instance {
    pub inputs: Vec<Vec<u32>>,
    pub outputs: Box<[u32]>,
}
pub struct Selection {
    instances: Vec<Instance>,
    pub solutions: Vec<Node>,
    params: Vec<Parametres>
}

#[derive(Copy, Clone, Debug)]
pub enum Greedy {
    ///Naif(tau)
    Naif(f32),
    ///Progressif(tau_min, tau_max)
    Progressif(f32, f32),

}
#[derive(Copy, Clone, Debug)]
pub enum Enumerator {
    ///Random(size, nombre d'itérations par formule, nombre de formules)
    Random(usize, usize, usize),
    ///ProgressiveSize(size_max, nombre d'itérations par formule, nombre de formules)
    ProgressiveSize(usize, usize, usize),
    ///ProgressiveSize(size_max, nombre d'itérations par formule)
    AlternateSize(usize, usize),
}

#[derive(Copy, Clone, Debug)]
pub struct Solver {
    pub enumerator: Enumerator,
    pub greedy: Greedy,
}

pub struct SolverResult {
    result: Option<Node>,
    time: u128,
}

pub enum FinalResult {
    Found(FoundResult),
    NotFound(JsonData, u128)
}

#[derive(Deserialize, Serialize)]
pub struct FoundResult {
    formula: String,
    time: u128,
    solution: String,
    size: usize,
    equivalence: f32,
    solver: String,
    param: Parametres,
    instance: Instance
}

impl FoundResult {
    pub fn from_str(json_str: &str) -> Self {
        let fr: Self = serde_json::from_str(json_str).unwrap();
        fr
    }

    pub fn to_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl FinalResult {
    pub fn from(solver_result: SolverResult, json_data: JsonData, solver: Solver, rng: &mut impl Rng) -> Self {
        let SolverResult { result, time } = solver_result;
        if let Some(mut formula) = result {
            let JsonData { instance, solution, param } = json_data;
            let (n_inputs, ..) = param;
            let equivalence = formula.compare(n_inputs, &mut Node::from_str(&solution, 1), rng);
            Self::Found(FoundResult { formula: formula.to_string(), time, solution, size: formula.size(), equivalence, solver: format!("{:?}", solver), param, instance })
        }
        else {
            Self::NotFound(json_data, time)
        }
    }
}

impl JsonData {
    pub fn random(params: Parametres, rng: &mut impl Rng) -> Self {
        let (n_inputs, n_examples, size) = params;
        let mut f = Node::random(n_inputs, size, 1, rng);
        let instance = f.to_instance(n_inputs, n_examples, rng);
        Self { instance, solution: f.to_string(), param: params }
    }

    pub fn from_file(filename: &str) -> Self {
        let mut data_file = File::open(filename).unwrap();
        let mut file_content = String::new();
        data_file.read_to_string(&mut file_content).unwrap();
        Self::from_str(file_content.as_str())
    }

    pub fn to_file(&self, filename: &str) {
        let mut data_file = File::create(filename).expect("creation failed");
        let buffer = self.to_str();
        data_file.write(buffer.as_bytes()).expect("write failed");
    }

    pub fn from_str(json_str: &str) -> Self {
        let jd: Self = serde_json::from_str(json_str).unwrap();
        jd
    }

    pub fn to_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn solve_final_result(self, solver: Solver, rng: &mut impl Rng) {
        let JsonData { instance, .. } = &self;
        let solver_result = instance.solve(solver, rng);
        println!("{}", FinalResult::from(solver_result, self, solver, rng))
    }

    pub fn solve_print(&self, solver: Solver, rng: &mut impl Rng) {
        let JsonData { instance, solution, param } = self;
        let mut node_solution = Node::from_str(solution.as_str(), 1);
        let (n_inputs, n_examples, size) = *param;
        let SolverResult { result: f, time } = instance.solve(solver, rng);
        println!();
        println!("{solver}");
        println!("Paramètres          : (n_inputs, n_examples, size) = ({n_inputs}, {n_examples}, {size})");
        println!("Temps de calcul     : {time} ms");
        if let Some(mut g) = f {
            println!("Formule solution    : {node_solution}");
            println!("Formule obtenue     : {g}");
            let s = g.compare(n_inputs, &mut node_solution, rng);
            println!("Score d'équivalence : {s}");
            println!("Taille de la formule: {}", g.size());
        } else {
            println!("Pas de formule trouvée");
        }
    }
}

impl Instance {
    pub fn random(params: Parametres, rng: &mut impl Rng) -> Instance {
        let (n_inputs, n_examples, size) = params;
        let mut f = Node::random(n_inputs, size, 1, rng);
        f.to_instance(n_inputs, n_examples, rng)
    }

    pub fn solve(&self, solver: Solver, rng: &mut impl Rng) -> SolverResult {
        solver.solve(&self, rng)
    }
}

impl Selection {
    pub fn new(param: Parametres, rng: &mut impl Rng, n: usize, filename: &str) -> Self {
        let mut data_file = File::create(filename).expect("creation failed");
        for _ in 0..n {
            let json_data= JsonData::random(param, rng);
            let mut buffer = json_data.to_str();
            buffer.push('\n');
            data_file.write(buffer.as_bytes()).expect("write failed");
        }
        Self::from_file(filename)
    }

    pub fn from_file(filename: &str) -> Self {
        let mut instances = Vec::new();
        let mut solutions = Vec::new();
        let mut params = Vec::new();
        let mut data_file = File::open(filename).unwrap();
        let mut file_content = String::new();
        data_file.read_to_string(&mut file_content).unwrap();
        for line in file_content.lines() {
            let JsonData { instance, solution, param } = JsonData::from_str(line);
            instances.push(instance);
            solutions.push(Node::from_str(solution.as_str(), 1));
            params.push(param);
        }
        Self { instances, solutions, params }
    }

    pub fn solve(&self, solver: Solver, rng: &mut impl Rng) -> Vec<SolverResult> {
        self.instances.iter()
            .enumerate()
            .map(|(i, instance)| {println!{"{i}"}; solver.solve(instance, rng)})
            .collect()
    }

    pub fn solve_print(&self, solver: Solver, rng: &mut impl Rng, params: Parametres, reset_selection: bool) {
        let (n_inputs, n_examples, size) = params;
        let now = Instant::now();
        let result = self.solve(solver, rng);
        let n = result.len();
        let total_time = now.elapsed().as_millis();
        let mut max_time: u128 = 0;
        let mut sum_size = 0;
        if reset_selection {Interpretor::new("interpretor.txt", n)};
        let mut interpretor = Interpretor::from_file("interpretor.txt");
        let s: usize = result.iter().enumerate().map(|(i, SolverResult {result, time})| {
            if *time > max_time {max_time += time};
            if let Some(f) = result {interpretor.update(i);sum_size += f.size(); 1} else {0}
        }).sum();
        interpretor.to_file("interpretor.txt");
        interpretor.print_in_file("results.txt");
        let mean_size = f32::value_from(sum_size).unwrap()/f32::value_from(s).unwrap();
        println!();
        println!("{solver}");
        println!("Paramètres                               : (n_inputs, n_examples, size) = ({n_inputs}, {n_examples}, {size})");
        println!("Nombre d'instances                       : {n}");
        println!("Nombre de succès                         : {s} / {n}");
        println!("Taille moyenne des formules obtenues     : {mean_size}");
        println!("Temps de calcul total                    : {total_time} ms");
        println!("Temps de calcul maximal pour une formule : {max_time} ms")
    }    

    pub fn change_instances(&mut self, rng: &mut impl Rng) {
        self.instances = self.solutions.iter_mut().enumerate().map(|(i, node)| {
            let (n_inputs, n_examples, _) = self.params[i];
            let i = node.to_instance(n_inputs, n_examples, rng);
            node.clear_forward();
            node.clear_backward();
            i
        }).collect()
    }
}

impl Greedy {
    pub fn solve(&self, instance: &Instance, rng: &mut impl Rng, f: &mut Node, n: usize) -> bool {
        let n_examples = instance.outputs.len();
        match self {
            Self::Naif(tau) => {
                self.naif(instance, n_examples, rng, n, *tau, f)
            } 
            Self::Progressif(tau_min, tau_max) => {
                self.progressif(instance, n_examples, rng, n, *tau_min, *tau_max, f)
            } 
        }
    }
    

    pub fn naif(&self, instance: &Instance, n_examples: usize, rng: &mut impl Rng, n: usize, tau: f32, f: &mut Node) -> bool {
        let mut scores: Scores;
        for _ in 0..n {
            let s = f.current_score(instance, n_examples);
            if s > 0.99f32 {return true} 

            scores = f.get_scores(instance, n_examples);
            let (id, op) = scores.softmax(tau, rng);
            f.update_gate(id, op);
        };
        let s = f.current_score(instance, n_examples);
        return s > 0.99f32
    }

    pub fn progressif(&self, instance: &Instance, n_examples: usize, rng: &mut impl Rng, n: usize, tau_min: f32, tau_max: f32, f: &mut Node) -> bool {
        let f32n = f32::value_from(n).unwrap();
        let step = 10.*(tau_max-tau_min)/(f32n-10.);
        let mut tau = tau_min;
        for tau in (0..(n/10)).map(|_| {tau += step; tau}) {
            if self.naif(instance, n_examples, rng, 1, tau, f) {return true}
        };
        false
    }
}

impl Enumerator {
    fn list_formula(&self, n_inputs: usize) -> EnumeratorIntoIterator {
        EnumeratorIntoIterator {enumerator: *self, n_inputs, compteur: 0}
    }
}

struct EnumeratorIntoIterator {
    enumerator: Enumerator,
    n_inputs: usize,
    compteur: usize
}

impl Iterator for EnumeratorIntoIterator {
    type Item = (Node, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rng();
        match self.enumerator {
            Enumerator::Random(size, m, n) => {
                if n == 0 {None} else {
                    self.enumerator = Enumerator::Random(size, m, n-1);
                    Some((Node::random(self.n_inputs, size, 1, &mut rng), m))
                }
            },
            Enumerator::ProgressiveSize(size_max, m, n) => {
                if self.compteur == n {None} else {
                    let formula_per_size = n/size_max;
                    self.compteur += 1 ;
                    let size = self.compteur / formula_per_size + 1;
                    Some((Node::random(self.n_inputs, size, 1, &mut rng), m))
                }
            },
            Enumerator::AlternateSize(size_max, m) => {
                if self.compteur == size_max {self.compteur = 1} else {self.compteur += 1}
                    Some((Node::random(self.n_inputs, self.compteur, 1, &mut rng), m))
            }
        }
    }
}

impl Solver {
    fn solve(&self, instance: &Instance, rng: &mut impl Rng) -> SolverResult {
        let n_inputs = instance.inputs[0].len();
        let now = Instant::now();
        for (mut f, n) in self.enumerator.list_formula(n_inputs) {
            if self.greedy.solve(instance, rng, &mut f, n) {return SolverResult {result: Some(f), time: now.elapsed().as_millis()}}
        }
        SolverResult {result: None, time: now.elapsed().as_millis()}
    }
}

impl Display for Greedy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Naif(tau) => write!(f, "algorithme naif avec tau = {tau}"),
            Self::Progressif(tau_min, tau_max) => write!(f, "algorithme progressif avec tau allant de {tau_min} à {tau_max}"),
        }
    }
}

impl Display for Enumerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Random(size, n, m) => write!(f, "énumeration aléatoire de {n} formules de taille {size} avec {m} itérations par formule"),
            Self::ProgressiveSize(size_max, m, n) => write!(f, "énumeration aléatoire de {m} formules de taille jusqu'à {size_max} avec {m} itérations par formule"),
            Self::AlternateSize(size_max, m) => write!(f, "énumeration aléatoire de formules dont la taille alterne entre 1 et {size_max} avec {m} itérations par formule"),
        }
    }
}

impl Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Solver :\n   - Enumerateur        : {}\n   - Algorithme glouton : {}", self.enumerator, self.greedy)
    }
}

impl Display for FinalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FinalResult::Found(found_result) => write!(f, "Found: {}", found_result.to_str()),
            FinalResult::NotFound(json_data, time) => write!(f, "Not found: {time}ms {}", json_data.to_str()),
        }
    }
}
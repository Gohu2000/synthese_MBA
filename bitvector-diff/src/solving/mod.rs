use crate::formula::{
    Node,
    grad::Scores,
};
use rand::{rng, Rng};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::File,
    io::{Write, Read},
    time::Instant,
};
use conv::ValueFrom;

/// (n_inputs, n_examples, size)
pub type Parametres = (usize, usize, usize);

#[derive(Deserialize, Serialize)]
pub struct Instance {
    pub inputs: Vec<Vec<u32>>,
    pub outputs: Box<[u32]>,
}
pub struct Benchmark {
    instances: Vec<Instance>,
}

#[derive(Copy, Clone)]
pub enum Greedy {
    ///Naif(tau)
    Naif(f32),
    ///Progressif(tau_min, tau_max)
    Progressif(f32, f32),

}
#[derive(Copy, Clone)]
pub enum Enumerator {
    ///Random(size, nombre d'itérations par formule, nombre de formules)
    Random(usize, usize, usize),
    ///ProgressiveSize(size_max, nombre d'itérations par formule, nombre de formules)
    ProgressiveSize(usize, usize, usize),
}

#[derive(Copy, Clone)]
pub struct Solver {
    pub enumerator: Enumerator,
    pub greedy: Greedy,
}

pub struct SolverResult {
    result: Option<Node>,
    time: u128,
}

impl Instance {
    pub fn random(params: Parametres, rng: &mut impl Rng) -> Instance {
        let (n_inputs, n_examples, size) = params;
        let mut f = Node::random(n_inputs.into(), size.into(), 1, rng);
        let (inputs, outputs) = f.to_instance(n_inputs.into(), n_examples.into(), rng);
        Instance {
            inputs,
            outputs
        }
    }

    pub fn from_json(json_str: &str) -> Instance {
        let i: Instance = serde_json::from_str(json_str).unwrap();
        i
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn solve(&self, solver: Solver, rng: &mut impl Rng) -> SolverResult {
        solver.solve(&self, rng)
    }

    pub fn solve_print(&self, solver: Solver, rng: &mut impl Rng, params: Parametres) {
        let (n_inputs, n_examples, size) = params;
        let SolverResult { result: f, time } = self.solve(solver, rng);
        println!("{solver}");
        println!("Paramètres          : (n_inputs, n_examples, size) = ({n_inputs}, {n_examples}, {size})");
        println!("Temps de calcul     : {time} ms");
        if let Some(mut g) = f {
            println!("Formule obtenue     : {g}");
            let s = g.current_score(self, n_examples);
            println!("Score de la formule : {s}");
        } else {
            println!("Pas de formule trouvée");
        }
    }
}

impl Benchmark {
    pub fn new(params: Parametres, rng: &mut impl Rng, n: usize) -> Benchmark {
        let mut instances = Vec::new();
        for _ in 0..n {
            instances.push(Instance::random(params, rng));
        }
        Benchmark { instances }
    }

    pub fn from_file(filename: &str) -> Benchmark {
        let mut instances = Vec::new();
        let mut data_file = File::open(filename).unwrap();
        let mut file_content = String::new();
        data_file.read_to_string(&mut file_content).unwrap();
        for line in file_content.lines() {
            instances.push(Instance::from_json(line));
        }
        Benchmark { instances }
    }

    pub fn to_file(&self, filename: &str) {
        let mut data_file = File::create(filename).expect("creation failed");
        for i in &self.instances {
            let mut buffer = i.to_json();
            buffer.push('\n');
            data_file.write(buffer.as_bytes()).expect("write failed");
        }
    }

    pub fn solve(&self, solver: Solver, rng: &mut impl Rng) -> Vec<SolverResult> {
        self.instances.iter()
            .enumerate()
            .map(|(i, instance)| {println!{"{i}"}; solver.solve(instance, rng)})
            .collect()
    }

    pub fn solve_print(&self, solver: Solver, rng: &mut impl Rng, params: Parametres) {
        let (n_inputs, n_examples, size) = params;
        let now = Instant::now();
        let result = self.solve(solver, rng);
        let total_time = now.elapsed().as_millis();
        let mut max_time: u128 = 0;
        let mut sum_size = 0;
        let s: usize = result.iter().map(|SolverResult {result, time}| {
            if *time > max_time {max_time += time};
            if let Some(f) = result {sum_size += f.size(); 1} else {0}
        }).sum();
        let mean_size = f32::value_from(sum_size).unwrap()/f32::value_from(s).unwrap();
        let n = result.len();
        println!();
        println!("{solver}");
        println!("Paramètres                               : (n_inputs, n_examples, size) = ({n_inputs}, {n_examples}, {size})");
        println!("Nombre d'instances                       : {n}");
        println!("Nombre de succès                         : {s} / {n}");
        println!("Taille moyenne des formules obtenues     : {mean_size}");
        println!("Temps de calcul total                    : {total_time} ms");
        println!("Temps de calcul maximal pour une formule : {max_time} ms")
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
            Self::Random(size, n, m) => write!(f, "énumeration aléatoire de {m} formules de taille {size} avec {n} itérations par formule"),
            Self::ProgressiveSize(size_max, m, n) => write!(f, "énumeration aléatoire de {m} formules de taille jusqu'à {size_max} avec {n} itérations par formule"),
        }
    }
}

impl Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Solver :\n   - Enumerateur        : {}\n   - Algorithme glouton : {}", self.enumerator, self.greedy)
    }
}
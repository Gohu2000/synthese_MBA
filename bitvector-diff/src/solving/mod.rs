use crate::formula::{
    Node,
    grad::Scores,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::File,
    io::{Write, Read},
    time::Instant,
};

type Parametres = (usize, usize, usize);

#[derive(Deserialize, Serialize)]
pub struct Instance {
    pub inputs: Vec<Vec<u32>>,
    pub outputs: Box<[u32]>,
}
pub struct Benchmark {
    instances: Vec<Instance>,
}

#[derive(Copy, Clone)]
pub enum Solver {
    Algonaif,
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

    pub fn solve(&self, solver: Solver, rng: &mut impl Rng) -> Node {
        solver.solve(&self, rng)
    }

    pub fn solve_print(&self, solver: Solver, rng: &mut impl Rng, params: Parametres) {
        let (n_inputs, n_examples, size) = params;
        let now = Instant::now();
        let mut f = self.solve(solver, rng);
        let time = now.elapsed().as_millis();
        let s = f.current_score(self, n_examples);
        println!("Paramètres          : (n_inputs, n_examples, size) = ({n_inputs}, {n_examples}, {size})");
        println!("Formule obtenue     : {f}");
        println!("Score de la formule : {s}");
        println!("Temps de calcul     : {time} ms")
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

    pub fn solve(&self, solver: Solver, rng: &mut impl Rng) -> Vec<Node> {
        self.instances.iter()
            .enumerate()
            .map(|(i, instance)| {println!{"{i}"}; solver.solve(instance, rng)})
            .collect()
    }

    pub fn solve_print(&self, solver: Solver, rng: &mut impl Rng, params: Parametres) {
        let (n_inputs, n_examples, size) = params;
        let now = Instant::now();
        let mut result = self.solve(solver, rng);
        let time = now.elapsed().as_millis();
        let s: usize = result.iter_mut().zip(&self.instances).map(|(formula, instance)| {
            if formula.current_score(instance, n_examples) > 0.99f32 {1} else {0}
        }).sum();
        let n = result.len();
        println!();
        println!("Paramètres          : (n_inputs, n_examples, size) = ({n_inputs}, {n_examples}, {size})");
        println!("Algorithme          : {solver}");
        println!("Nombre d'instances  : {n}");
        println!("Nombre de succès    : {s} / {n}");
        println!("Temps de calcul     : {time} ms")
    }    
}

impl Solver {
    pub fn solve(&self, instance: &Instance, rng: &mut impl Rng) -> Node{
        let n_examples = instance.outputs.len();
        let n_inputs = instance.inputs[0].len();
        match self {
            Solver::Algonaif => {
                self.algonaif(instance, n_examples, n_inputs, rng, 1000)
            } 
        }
    }

    pub fn algonaif(&self, instance: &Instance, n_examples: usize, n_inputs: usize, rng: &mut impl Rng, n: usize) -> Node {
        let mut f= Node::random(n_inputs, 15, 1, rng);
        let mut scores: Scores;
        for _ in 0..n {
            let s = f.current_score(instance, n_examples);
            if s > 0.99f32 {return f}

            scores = f.get_scores(instance, n_examples);
            let (id, op) = scores.softmax(20., rng);
            f.update_gate(id, op);
        }
        f
    }
}

impl Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solver::Algonaif => write!(f, "algorithme naif"),
        }
    }
}
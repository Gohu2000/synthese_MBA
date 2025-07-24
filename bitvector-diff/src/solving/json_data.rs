use std::{collections::HashMap, fs::File, io::{Read, Write}, fmt::LowerHex};

use clap::builder::Str;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{formula::Node, solving::{results::FinalResult, Instance, Parametres, Solver, SolverResult}};

#[derive(Deserialize, Serialize)]
pub struct JsonData {
    pub instance: Instance,
    pub solution: String,
    pub param: Parametres
}

impl JsonData {
    pub fn random(params: Parametres, rng: &mut impl Rng, with_shift: bool) -> Self {
        let (n_inputs, n_examples, size) = params;
        let mut f = Node::random(n_inputs, size, 1, rng, with_shift);
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

    pub fn solve_final_result(self, max_time: u64, solver: Solver, rng: &mut impl Rng, with_shift: bool) {
        let JsonData { instance, .. } = &self;
        let solver_result = instance.solve(max_time, solver, rng, with_shift);
        println!("{}", FinalResult::from(solver_result, self, solver, rng))
    }

    pub fn solve_print(&self, max_time: u64, solver: Solver, rng: &mut impl Rng, with_shift: bool) {
        let JsonData { instance, solution, param } = self;
        let mut node_solution = Node::from_str(solution.as_str(), 1);
        let (n_inputs, n_examples, size) = *param;
        let SolverResult { result: f, time } = instance.solve(max_time, solver, rng, with_shift);
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

#[derive(Deserialize, Serialize)]
pub struct XyntiaJsonData {
    initial: XyntiaInstance,
    sampling: HashMap<usize, XyntiaInstance>
}

#[derive(Deserialize, Serialize)]
pub struct XyntiaInstance {
    inputs: HashMap<usize, XyntiaIO>,
    outputs: HashMap<usize, XyntiaIO>,
}

#[derive(Deserialize, Serialize)]
pub struct XyntiaIO {
    location: String,
    size: String,
    value: String,
}

impl XyntiaJsonData {
    pub fn from(Instance {inputs, outputs}: &Instance) -> Self {
        let initial = XyntiaInstance::new(inputs[0].clone(), outputs[0]);
        let mut sampling = HashMap::new();
        for i in 1..outputs.len() {
            sampling.insert(i-1, XyntiaInstance::new(inputs[i].clone(), outputs[i]));
        }
        Self { initial, sampling}
    }

    pub fn to_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    
    pub fn to_file(&self, filename: &str) {
        let mut data_file = File::create(filename).expect("creation failed");
        let buffer = self.to_str();
        data_file.write(buffer.as_bytes()).expect("write failed");
    }
}

impl XyntiaInstance {
    pub fn new(input: Vec<u32>, output: u32) -> Self {
        let mut inputs = HashMap::new();
        for (i, x) in input.iter().enumerate() {
            inputs.insert(i, XyntiaIO::new(*x, format!("mem{i}")));
        }
        let mut outputs = HashMap::new();
        outputs.insert(0, XyntiaIO::new(output, String::from("EAX")));
        XyntiaInstance { inputs, outputs}
    }
}

impl XyntiaIO {
    pub fn new(value: u32, location: String) -> Self {
        XyntiaIO { location, size: String::from("0x20"), value: format!("{value:#x}") }
    }
}
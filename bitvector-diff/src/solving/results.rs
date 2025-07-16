use std::{
    fs::File,
    io::{Write, Read},
    fmt::Display,
};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{formula::Node, solving::{Instance, JsonData, Parametres, Solver, SolverResult}};

pub enum FinalResult {
    Found(FoundResult),
    NotFound(JsonData, u128)
}

#[derive(Deserialize, Serialize)]
pub struct FoundResult {
    pub formula: String,
    pub time: u128,
    pub solution: String,
    pub size: usize,
    pub equivalence: f32,
    pub solver: String,
    pub param: Parametres,
    pub instance: Instance
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

    pub fn from_str(line: &str) -> Self {
        if line.starts_with("Found: ") {
            let json_str = &line[7..];
            Self::Found(FoundResult::from_str(json_str))
        }
        else {
            let mut indice_json_str = 0;
            let time= {
                let mut buffer = String::new();
                for (i, c) in line[11..].chars().enumerate() {
                    if c == 'm' {
                        indice_json_str = i + 14;
                        break
                    }
                    else {
                        buffer.push(c);
                    }
                }
                buffer.parse().expect("Echec de la conversion")
            };
            let json_str = &line[indice_json_str..];
            Self::NotFound(JsonData::from_str(json_str), time)
        }
    }
}

pub struct Interpretor {
    found_vec: Vec<usize>
}

impl Interpretor {
    pub fn new(filename: &str, n: usize) {
        let mut interpretor = Self { found_vec: Vec::new()};
        for _ in 0..n {interpretor.found_vec.push(0)}
        interpretor.to_file(filename)
    }

    pub fn update(&mut self, i: usize) {
        self.found_vec[i] += 1 
    }

    pub fn to_file(&self, filename: &str) {
        let mut data_file = File::create(filename).expect("creation failed");
        let buffer = serde_json::to_string(&self.found_vec).unwrap();
        data_file.write(buffer.as_bytes()).expect("write failed");
    }

    pub fn from_file(filename: &str) -> Self {
        let mut data_file = File::open(filename).unwrap();
        let mut file_content = String::new();
        data_file.read_to_string(&mut file_content).unwrap();
        let vec: Vec<usize> = serde_json::from_str(file_content.as_str()).unwrap();
        Self { found_vec: vec }
    }

    pub fn print_in_file(&self, filename: &str) {
        let mut data_file = File::create(filename).expect("creation failed");
        let mut buffer = String::new();
        for (i, counter) in self.found_vec.iter().enumerate() {
            buffer.push_str(counter.to_string().as_str());
            if (i+1) % 10 == 0 {buffer.push('\n')} else {buffer.push(' ')}
        }
        data_file.write(buffer.as_bytes()).expect("write failed");
    }

    pub fn print_if_counter(&self, formulas: Vec<Node>, n: usize) {
        for (i, counter) in self.found_vec.iter().enumerate() {
            if *counter == n {
                println!("{}", formulas[i])
            }
        }
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
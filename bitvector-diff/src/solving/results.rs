use std::{
    fs::File,
    io::{Write, Read},
};

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
}
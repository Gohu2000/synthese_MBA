use std::collections::HashMap;
use fast_math::exp;
use rand::Rng;
use conv::ValueFrom;

use crate::formula::{binary::BinaryOp, unary::UnaryOp, Op};

#[derive(Debug, Clone, Copy)]
pub struct Grad {
    pub influence: u32,
    pub target: u32,
}

#[derive(Debug)]
pub struct Scores {
    pub values: HashMap<usize, Deltas>,
}

#[derive(Debug)]
pub enum Deltas {
    Input(HashMap<usize, i32>),
    Unary(HashMap<UnaryOp, i32>),
    Binary(HashMap<BinaryOp, i32>),
}

impl Scores {
    pub fn new() -> Self {
        Scores {
            values: HashMap::new()
        }
    }

    pub fn softmax(&self, n_examples: usize ,tau:f32, rng: &mut impl Rng) -> (usize, Op) {
        let f = |v: i32| {exp(tau*(v as f32)/(32.*f32::value_from(n_examples).unwrap()))};
        let mut sum = 0f32;
        let mut hashmapsum = HashMap::new();
        for (id, delta) in &self.values {
            let sum_of_id = delta.sum(f);

            hashmapsum.insert(*id, sum_of_id);
            sum += sum_of_id;
        };
        let mut p: f32 = rng.random();
        let mut sum_of_id;
        for (id, delta) in &self.values {
            sum_of_id = *hashmapsum.get(id).unwrap();
            p -= sum_of_id / sum;
            if p <= 0.002 {
                return (*id, delta.get_op(p + sum_of_id / sum, |v| {f(v)/sum}))
            }
        }
        panic!("{p}")
    }
}

impl Deltas {
    fn sum(&self, f: impl Fn(i32) -> f32) -> f32 {
        match self {
            Deltas::Input(h) => {
                let mut sum = 0f32;
                for (_, v) in h {
                    sum = sum + f(*v)
                }
                sum
            },
            Deltas::Unary(h) => {
                let mut sum = 0f32;
                for (_, v) in h {
                    sum = sum + f(*v)
                }
                sum
            },
            Deltas::Binary(h) => {
                let mut sum = 0f32;
                for (_, v) in h {
                    sum = sum + f(*v)
                }
                sum
            },
        }
    }

    fn get_op(&self, mut p:f32, f: impl Fn(i32) -> f32) -> Op {
        match self {
            Deltas::Input(h) => {
                for (op, v) in h {
                    p -= f(*v);
                    if p <= 0.002 {
                        return Op::Input(*op)
                    }
                }
                println!("{p}");
                panic!("{p}")
            },
            Deltas::Unary(h) => {
                for (op, v) in h {
                    p -= f(*v);
                    if p <= 0.002 {
                        return Op::Unary(*op)
                    }
                }
                println!("{p}");
                panic!("{p}")
            },
            Deltas::Binary(h) => {
                for (op, v) in h {
                    p -= f(*v);
                    if p <= 0.002 {
                        return Op::Binary(*op)
                    }
                }
                println!("{p}");
                panic!("{p}")
            },
        }
    }
}
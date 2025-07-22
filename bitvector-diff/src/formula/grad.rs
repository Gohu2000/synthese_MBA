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

pub struct Scores {
    pub values: HashMap<usize, Deltas>,
}
pub enum Deltas {
    Input(HashMap<usize, u32>),
    Unary(HashMap<UnaryOp, u32>),
    Binary(HashMap<BinaryOp, u32>),
}

impl Scores {
    pub fn new() -> Self {
        Scores {
            values: HashMap::new()
        }
    }

    pub fn softmax(&self, n_examples: usize ,tau:f32, rng: &mut impl Rng) -> (usize, Op) {
        let f = |v: u32| {exp(tau*(v as f32)/(32.*f32::value_from(n_examples).unwrap()))};
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
    fn sum(&self, f: impl Fn(u32) -> f32) -> f32 {
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

    fn get_op(&self, mut p:f32, f: impl Fn(u32) -> f32) -> Op {
        match self {
            Deltas::Input(h) => {
                for (op, v) in h {
                    p -= f(*v);
                    if p <= 0.002 {
                        return Op::Input(*op)
                    }
                }
                panic!("{p}")
            },
            Deltas::Unary(h) => {
                for (op, v) in h {
                    p -= f(*v);
                    if p <= 0.002 {
                        return Op::Unary(*op)
                    }
                }
                panic!("{p}")
            },
            Deltas::Binary(h) => {
                for (op, v) in h {
                    p -= f(*v);
                    if p <= 0.002 {
                        return Op::Binary(*op)
                    }
                }
                panic!("{p}")
            },
        }
    }
}
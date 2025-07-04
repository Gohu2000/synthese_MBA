use std::collections::HashMap;
use fast_math::exp;
use rand::Rng;

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
    Input(HashMap<usize, f32>),
    Unary(HashMap<UnaryOp, f32>),
    Binary(HashMap<BinaryOp, f32>),
}

impl Scores {
    pub fn new() -> Self {
        Scores {
            values: HashMap::new()
        }
    }

    pub fn softmax(&self, tau:f32, rng: &mut impl Rng) -> (usize, Op) {
        let f = |v: f32| {exp(tau*v)};
        let mut sum = 0f32;
        let mut hashmapsum = HashMap::new();
        for (id, delta) in &self.values {
            let id_sum = delta.sum(f);
            hashmapsum.insert(*id, id_sum);
            sum += id_sum;
        };
         let mut p: f32 = rng.random();
        for (id, delta) in &self.values {
            let id_sum = *hashmapsum.get(id).unwrap();
            p -= id_sum / sum;
            if p <= 0.001 {
                return (*id, delta.get_op(p + id_sum / sum, |v| {f(v)/sum}))
            }
        }
        panic!("{p}")
    }
}

impl Deltas {
    fn sum(&self, f: impl Fn(f32) -> f32) -> f32 {
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

    fn get_op(&self, mut p:f32, f: impl Fn(f32) -> f32) -> Op {
        match self {
            Deltas::Input(h) => {
                for (op, v) in h {
                    p -= f(*v);
                    if p <= 0.001 {
                        return Op::Input(*op)
                    }
                }
                panic!("{p}")
            },
            Deltas::Unary(h) => {
                for (op, v) in h {
                    p -= f(*v);
                    if p <= 0.001 {
                        return Op::Unary(*op)
                    }
                }
                panic!("{p}")
            },
            Deltas::Binary(h) => {
                for (op, v) in h {
                    p -= f(*v);
                    if p <= 0.001 {
                        return Op::Binary(*op)
                    }
                }
                panic!("{p}")
            },
        }
    }
}
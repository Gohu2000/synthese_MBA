use std::collections::HashMap;

use crate::formula::{binary::BinaryOp, unary::UnaryOp};

#[derive(Debug, Clone, Copy)]
pub struct Grad {
    pub influence: u32,
    pub target: u32,
}

pub struct Scores {
    values: HashMap<usize, Deltas>,
}

pub enum Deltas {
    Input(HashMap<usize, isize>),
    Unary(HashMap<UnaryOp, isize>),
    Binary(HashMap<BinaryOp, isize>),
}

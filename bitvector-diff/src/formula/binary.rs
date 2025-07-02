use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, BitXor, Not},
};

use rand::{
    distr::{Distribution, StandardUniform},
    seq::IndexedRandom,
};

use super::grad::Grad;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    And,
    Or,
    Xor,
}

impl BinaryOp {
    pub fn apply(&self, x: u32, y: u32) -> u32 {
        match self {
            BinaryOp::And => x.bitand(y),
            BinaryOp::Or => x.bitor(y),
            BinaryOp::Xor => x.bitxor(y),
        }
    }

    pub fn grad_x(&self, Grad { influence, target }: Grad, y: u32) -> Grad {
        match self {
            BinaryOp::And => Grad {
                influence: influence.bitand(y),
                target,
            },
            BinaryOp::Or => Grad {
                influence: influence.bitand(y.not()),
                target,
            },
            BinaryOp::Xor => Grad {
                influence,
                target: target.bitxor(y),
            },
        }
    }

    pub fn grad_y(&self, Grad { influence, target }: Grad, y: u32) -> Grad {
        match self {
            BinaryOp::And => Grad {
                influence: influence.bitand(y),
                target,
            },
            BinaryOp::Or => Grad {
                influence: influence.bitand(y.not()),
                target,
            },
            BinaryOp::Xor => Grad {
                influence,
                target: target.bitxor(y),
            },
        }
    }
}

impl IntoIterator for BinaryOp {
    type Item = BinaryOp;
    type IntoIter = BinaryOpIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        BinaryOpIntoIterator {
            unwanted_op: self,
            index: 0,
        }
    }
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::And => write!(f, "&"),
            BinaryOp::Or => write!(f, "|"),
            BinaryOp::Xor => write!(f, "^"),
        }
    }
}

impl Distribution<BinaryOp> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> BinaryOp {
        [BinaryOp::And, BinaryOp::Or, BinaryOp::Xor]
            .choose(rng)
            .copied()
            .unwrap()
    }
}
pub struct BinaryOpIntoIterator {
    unwanted_op: BinaryOp,
    index: usize,
}

impl Iterator for BinaryOpIntoIterator {
    type Item = BinaryOp;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => BinaryOp::And,
            1 => BinaryOp::Or,
            2 => BinaryOp::Xor,
            _ => return None
        };
        self.index += 1;
        if result == self.unwanted_op {self.next()} else {Some(result)}
    }
}
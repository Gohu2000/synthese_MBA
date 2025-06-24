use std::ops::Not;

use rand::distr::{Distribution, StandardUniform};

use super::grad::Grad;

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Not,
    LeftShift(u32),
    RightShift(u32),
}

impl UnaryOp {
    pub fn apply(&self, x: u32) -> u32 {
        match self {
            UnaryOp::Not => x.not(),
            UnaryOp::LeftShift(i) => x.unbounded_shl(*i),
            UnaryOp::RightShift(i) => x.unbounded_shr(*i),
        }
    }

    pub fn grad(&self, Grad { influence, target }: Grad) -> Grad {
        match self {
            UnaryOp::Not => Grad {
                influence,
                target: target.not(),
            },
            UnaryOp::LeftShift(i) => Grad {
                influence: influence.unbounded_shl(*i),
                target: target.unbounded_shl(*i),
            },
            UnaryOp::RightShift(i) => Grad {
                influence: influence.unbounded_shr(*i),
                target: target.unbounded_shr(*i),
            },
        }
    }
}

impl Distribution<UnaryOp> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> UnaryOp {
        match rng.random_range(0..3) {
            0 => UnaryOp::Not,
            1 => UnaryOp::LeftShift(rng.random_range(0..=32)),
            2 => UnaryOp::RightShift(rng.random_range(0..=32)),
            _ => unreachable!(),
        }
    }
}

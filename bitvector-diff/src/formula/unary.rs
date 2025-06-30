use std::{fmt::Display, ops::Not};

use rand::distr::{Distribution, StandardUniform};

use super::grad::Grad;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
                influence: influence.unbounded_shr(*i),
                target: target.unbounded_shr(*i),
            },
            UnaryOp::RightShift(i) => Grad {
                influence: influence.unbounded_shl(*i),
                target: target.unbounded_shl(*i),
            },
        }
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Not => write!(f, "!"),
            UnaryOp::LeftShift(i) => write!(f, "<< {i}"),
            UnaryOp::RightShift(i) => write!(f, ">> {i}"),
        }
    }
}

impl IntoIterator for UnaryOp {
    type Item = UnaryOp;
    type IntoIter = UnaryOpIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        UnaryOpIntoIterator {
            unwanted_op: self,
            op: UnaryOp::Not,
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

pub struct UnaryOpIntoIterator {
    unwanted_op: UnaryOp,
    op: UnaryOp,
}

impl Iterator for UnaryOpIntoIterator {
    type Item = UnaryOp;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.op; 
        match self.op {
            UnaryOp::Not => self.op = UnaryOp::LeftShift(0),
            UnaryOp::LeftShift(31) => self.op = UnaryOp::RightShift(0),
            UnaryOp::LeftShift(x) => self.op = UnaryOp::LeftShift(x+1),
            UnaryOp::RightShift(32) => return None,
            UnaryOp::RightShift(x) => self.op = UnaryOp::RightShift(x+1),
             
        };
        if result == self.unwanted_op {self.next()} else {Some(result)}
    }
}

#[cfg(test)]
mod test {
    use rand::{Rng, rng};

    use crate::formula::unary::UnaryOp;

    #[test]
    fn not_not_is_id() {
        for _ in 0..100 {
            let x: u32 = rng().random();
            let op = UnaryOp::Not;
            let y = op.apply(x);
            let z = op.apply(y);
            assert_eq!(x, z);
        }
    }

    #[test]
    fn shift_left_right() {
        for _ in 0..100 {
            let x: u32 = rng().random();
            let dist = rng().random_range(0..32);
            let shift_left = UnaryOp::LeftShift(dist);
            let shift_right = UnaryOp::RightShift(dist);
            let y = shift_left.apply(x);
            let z = shift_right.apply(y);
            assert!(z <= x);
            assert_eq!(x & z, z);
        }
    }
}

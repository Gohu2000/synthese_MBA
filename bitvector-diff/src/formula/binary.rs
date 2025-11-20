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
    pub fn from_char(c:char) -> Result<Self, &'static str> {
        match c {
            '^' => Ok(BinaryOp::Xor),
            '|' => Ok(BinaryOp::Or),
            '&' => Ok(BinaryOp::And),
            _ => Err("Le charactère ne correspond pas à une opération binaire.")
        }
    }

    pub fn into_iter_others(self) -> BinaryOpIntoIterator {
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

#[cfg(test)]
mod test {
    use rand::{Rng, rng};

    use crate::formula::binary::BinaryOp;
    use crate::formula::unary::UnaryOp;

    #[test]
    fn xor_exchange() {
        for _ in 0..100 {
            let a: u32 = rng().random();
            let b: u32 = rng().random();
            let mut x: u32 = a;
            let mut y: u32 = b;
            let op = BinaryOp::Xor;
            x = op.apply(x, y);
            y = op.apply(x, y);
            x = op.apply(x, y);
            assert_eq!(x, b);
            assert_eq!(y, a);
        }
    }

    #[test]
    fn a_xor_a_is_0() {
        for _ in 0..100 {
            let a: u32 = rng().random();
            let op = BinaryOp::Xor;
            let x = op.apply(a, a);
            assert_eq!(x, 0);
        }
    }

    #[test]
    fn and_0_is_0() {
        for _ in 0..100 {
            let a: u32 = rng().random();
            let op = BinaryOp::And;
            let x = op.apply(a, 0);
            assert_eq!(x, 0);
        }
    }

    #[test]
    fn and_max_is_id() {
        for _ in 0..100 {
            let max = u32::max_value();
            let a: u32 = rng().random();
            let op = BinaryOp::And;
            let x = op.apply(a, max);
            assert_eq!(x, a);
        }
    }

    #[test]
    fn or_0_is_id() {
        for _ in 0..100 {
            let a: u32 = rng().random();
            let op = BinaryOp::Or;
            let x = op.apply(a, 0);
            assert_eq!(x, a);
        }
    }

    #[test]
    fn or_max_is_max() {
        for _ in 0..100 {
            let max = u32::max_value();
            let a: u32 = rng().random();
            let op = BinaryOp::Or;
            let x = op.apply(a, max);
            assert_eq!(x, max);
        }
    }

    #[test]
    fn morgan_1() {
        for _ in 0..100 {
            let a: u32 = rng().random();
            let b: u32 = rng().random();
            let and = BinaryOp::And;
            let or = BinaryOp::Or;
            let not = UnaryOp::Not;
            let x = and.apply(not.apply(a), not.apply(b));
            let y = not.apply(or.apply(a, b));
            assert_eq!(x, y);
        }  
    }

    #[test]
    fn morgan_2() {
        for _ in 0..100 {
            let a: u32 = rng().random();
            let b: u32 = rng().random();
            let and = BinaryOp::And;
            let or = BinaryOp::Or;
            let not = UnaryOp::Not;
            let x = or.apply(not.apply(a), not.apply(b));
            let y = not.apply(and.apply(a, b));
            assert_eq!(x, y);
        }  
    }
}

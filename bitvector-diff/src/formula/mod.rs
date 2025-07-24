use std::{
    collections::HashMap, fmt::{Debug, Display}, num::NonZeroUsize, ops::{BitAnd, BitXor, Not}, vec
};

use rand::Rng;

use crate::{formula::{
    binary::BinaryOp,
    grad::{Deltas, Grad, Scores},
    unary::UnaryOp,
}, solving::Instance};

mod binary;
pub mod grad;
mod unary;

use conv::ValueFrom;

#[derive(Debug)]
pub struct Node {
    /// Nodes have non-zero indices: this is makes navigating the tree easier.
    id: NonZeroUsize,
    data: NodeContent,
    outputs: Option<Box<[u32]>>,
    grads: Option<Box<[Grad]>>,
}

#[derive(Debug)]
enum NodeContent {
    Input(usize),
    UnaryNode {
        op: UnaryOp,
        child: Box<Node>,
    },

    BinaryNode {
        op: BinaryOp,
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl NodeContent {
    pub fn forward(&mut self, inputs: &[Vec<u32>]) -> Box<[u32]> {
        match self {
            NodeContent::Input(i) => inputs.iter().map(|input| input[*i]).collect(),
            NodeContent::UnaryNode { op, child } => {
                let child_out = child.forward(inputs);
                child_out.iter().map(|x| op.apply(*x)).collect()
            }
            NodeContent::BinaryNode { op, left, right } => {
                let left_out = left.forward(inputs);
                let right_out = right.forward(inputs);
                left_out
                    .iter()
                    .zip(right_out)
                    .map(|(x, y)| op.apply(*x, *y))
                    .collect()
            }
        }
    }

    pub fn backward(&mut self, grads: &[Grad]) {
        match self {
            NodeContent::Input(_) => (),
            NodeContent::UnaryNode { op, child } => {
                let child_grads = grads.iter().copied().map(|g| op.grad(g)).collect();
                child.backward(child_grads);
            }
            NodeContent::BinaryNode { op, left, right } => {
                let right_values = right
                    .outputs
                    .as_ref()
                    .expect("'backward' called on a node with a child without outputs.");
                let left_grads = grads
                    .iter()
                    .zip(right_values)
                    .map(|(g, y)| op.grad_x(*g, *y))
                    .collect();
                left.backward(left_grads);

                let left_values = left
                    .outputs
                    .as_ref()
                    .expect("'backward' called on a node with a child without outputs.");
                let right_grads = grads
                    .iter()
                    .zip(left_values)
                    .map(|(g, x)| op.grad_y(*g, *x))
                    .collect();
                right.backward(right_grads);
            }
        }
    }

    pub fn clear_backward(&mut self) {
        match self {
            NodeContent::Input(_) => (),
            NodeContent::UnaryNode { op:_ , child } => {
                child.clear_backward();
            }
            NodeContent::BinaryNode { op:_ , left, right } => {
                left.clear_backward();
                right.clear_backward();
            }
        }
    }

    pub fn clear_forward(&mut self) {
        match self {
            NodeContent::Input(_) => (),
            NodeContent::UnaryNode { op:_ , child } => {
                child.clear_forward();
            }
            NodeContent::BinaryNode { op:_ , left, right } => {
                left.clear_forward();
                right.clear_forward();
            }
        }
    }

    fn _walk(&mut self, m: Move, moves: Vec<Move>) -> &mut Node {
        match self {
            NodeContent::Input(_) => unreachable!(),
            NodeContent::UnaryNode { child, .. } => {
                assert_eq!(m, Move::Left);
                child._walk(moves)
            }
            NodeContent::BinaryNode { left, right, .. } => match m {
                Move::Left => left._walk(moves),
                Move::Right => right._walk(moves),
            },
        }
    }

    fn walk_outputs(&mut self, m: Move, moves: Vec<Move>) -> &mut Node {
        match self {
            NodeContent::Input(_) => unreachable!(),
            NodeContent::UnaryNode { child, .. } => {
                assert_eq!(m, Move::Left);
                child.walk_outputs(moves)
            }
            NodeContent::BinaryNode { left, right, .. } => match m {
                Move::Left => left.walk_outputs(moves),
                Move::Right => right.walk_outputs(moves),
            },
        }
    }

    fn set_op(&mut self, op: Op) {
        match (self, op) {
            (NodeContent::Input(i), Op::Input(j)) => *i = j,
            (NodeContent::UnaryNode { op, .. }, Op::Unary(unary_op)) => *op = unary_op,
            (NodeContent::BinaryNode { op, .. }, Op::Binary(binary_op)) => *op = binary_op,
            (s, op) => panic!("Invalid pair of (NodeContent, op): '{s:?}', '{op:?}'"),
        }
    }

    fn compute_deltas(&mut self, grads: &[Grad], outputs: &[u32], inputs: &[Vec<u32>], with_shift: bool) -> Deltas {
        let p = |n: u32| {u32::count_ones(n) as i32};
        let compute_ratio_correct_bits = |output: u32, Grad {influence, target}| {
            p((output.bitxor(target)).not().bitand(influence))
        };
        match self {
            NodeContent::Input(i) => {
                let n_inputs = inputs[0].len();
                let mut hashmap = HashMap::new();
                for j in 0..n_inputs {
                    if j != *i {
                        let v = inputs.iter().enumerate().map(|(k, input)| {
                            compute_ratio_correct_bits(input[j], grads[k]) - compute_ratio_correct_bits(outputs[k], grads[k])
                        }).sum();
                        hashmap.insert(j, v);
                    }
                }
                Deltas::Input(hashmap)
            },
            NodeContent::UnaryNode { op, child } => {
                let child_out = child.forward(inputs);
                let mut hashmap = HashMap::new();
                for new_op in op.into_iter_others(with_shift) {
                    let v = child_out.iter().enumerate().map(|(k, x)| {
                        compute_ratio_correct_bits(new_op.apply(*x), grads[k]) - compute_ratio_correct_bits(outputs[k], grads[k])
                    }).sum();
                    hashmap.insert(new_op, v);
                }
                Deltas::Unary(hashmap)
            },
            NodeContent::BinaryNode { op, left, right } => {
                let left_out = left.forward(inputs);
                let right_out = right.forward(inputs);
                let mut hashmap = HashMap::new();
                for new_op in op.into_iter_others() {
                    let v = left_out.iter().enumerate().map(|(k, x)| {
                        compute_ratio_correct_bits(new_op.apply(*x, right_out[k]), grads[k]) - compute_ratio_correct_bits(outputs[k], grads[k])
                    }).sum();
                    hashmap.insert(new_op, v);
                }
                Deltas::Binary(hashmap)
            },
        }
    }

    fn compute_scores(&mut self, inputs: &[Vec<u32>], scores: &mut Scores, with_shift: bool) {
        match self {
            NodeContent::Input(_) => (),
            NodeContent::UnaryNode { child, .. } => {
                child.compute_scores(inputs, scores, with_shift)
            }
            NodeContent::BinaryNode { left, right, .. } => {
                left.compute_scores(inputs, scores, with_shift);
                right.compute_scores(inputs, scores, with_shift);
            },
        }
    }

    fn size(&self) -> usize {
        match self {
            NodeContent::Input(_) => 1,
            NodeContent::UnaryNode { child, .. } => {
                child.size() + 1
            }
            NodeContent::BinaryNode { left, right, .. } => {
                left.size() + right.size() + 1
            },
        }
    }
}

impl Node {
    fn new<T: TryInto<NonZeroUsize>>(id: T, data: NodeContent) -> Self
    where
        T::Error: Debug,
    {
        Self {
            id: id.try_into().expect("Expected non-zero id"),
            data,
            outputs: None,
            grads: None,
        }
    }
    pub fn forward<'s, 'i>(&'s mut self, inputs: &'i [Vec<u32>]) -> &'s [u32] {
        self.outputs
            .get_or_insert_with(|| self.data.forward(inputs))
    }

    pub fn backward(&mut self, grads: Box<[Grad]>) {
        self.grads.get_or_insert_with(|| {
            self.data.backward(&grads);
            grads
        });
    }

    pub fn clear_backward(&mut self) {
        self.grads = None;
        self.data.clear_backward()
    }

    pub fn clear_forward(&mut self) {
        self.outputs = None;
        self.data.clear_forward()
    }

    pub fn random(n_inputs: usize, size: usize, id: usize, rng: &mut impl Rng, with_shift: bool) -> Self {
        if size == 1 {
            Node::new(id, NodeContent::Input(rng.random_range(0..n_inputs)))
        } else {
            let is_unary = rng.random_bool(0.25) || size == 2; // TODO: change proba ?
            if is_unary {
                let child = Self::random(n_inputs, size - 1, 2 * id, rng, with_shift);
                let data = NodeContent::UnaryNode {
                    op: if with_shift {rng.random()} else {UnaryOp::Not},
                    child: Box::from(child),
                };
                Node::new(id, data)
            } else {
                let left_size = rng.random_range(1..=(size - 2));
                let left_child = Self::random(n_inputs, left_size, 2 * id, rng, with_shift);
                let right_size = size - 1 - left_size;
                let right_child = Self::random(n_inputs, right_size, 2 * id + 1, rng, with_shift);
                let data = NodeContent::BinaryNode {
                    op: rng.random(),
                    left: Box::from(left_child),
                    right: Box::from(right_child),
                };
                Node::new(id, data)
            }
        }
    }

    pub fn from_str(str: &str, id: usize) -> Self {
        let mut counter = 0;
        for (i,c) in str.char_indices() {
            if (counter == 0) & (c == 'x') {
                if let Ok(j) = usize::from_str_radix(&str[2..], 10) {
                    return Node::new(id, NodeContent::Input(j))
                }
                else {panic!("wrong format for a formula : {str}")}
            }
            if c == '(' {counter += 1}
            if c == ')' {counter -= 1}
            if counter == 0 {
                if let Some(char_op) = str.chars().nth(i+2) {
                    match char_op {
                        '^' | '|' | '&' => {
                            let left_child = Node::from_str(&str[1..i], 2 * id);
                            let right_child = Node::from_str(&str[i+5..str.len()-1], 2 * id + 1);
                            return Node::new(id, NodeContent::BinaryNode {
                                op: BinaryOp::from_char(char_op).unwrap(),
                                left: Box::from(left_child),
                                right: Box::from(right_child)
                            })
                        },
                        '!' | '<' | '>' => {
                            let child = Node::from_str(&str[1..i], 2 * id);
                            let i_opt = if char_op == '!' {None} else {Some(&str[i+5..])};
                            return Node::new(id, NodeContent::UnaryNode {
                                op: UnaryOp::from_char(char_op, i_opt).unwrap(),
                                child: Box::from(child)
                            })
                        },
                        _ => panic!("wrong format for a formula : {str}")
                    }
                }
                else {panic!("wrong format for a formula : {str}")}
            }
        }
        panic!("wrong format for a formula : {str}")
    }

    pub fn to_instance(&mut self, n_inputs: usize, n_examples: usize, rng: &mut impl Rng) -> Instance {
        let mut vec = Vec::new();
        for _ in 0..n_examples {
            let x: Vec<u32> = (0..n_inputs).map(|_| rng.random()).collect();
            vec.push(x);
        };
        let outputs = self.data.forward(vec.as_slice());
        Instance {
            inputs: vec,
            outputs
        }
    }

    pub fn mutate(&mut self, n_inputs: usize, equilibrate: bool, rng: &mut impl Rng, with_shift: bool) {
        let id_leave = self.choose_leave(equilibrate, rng);
        let moves = moves_for_id(id_leave);
        let leave = self.walk_outputs(moves);
        let is_unary = rng.random_bool(0.5);
        if is_unary {
            let child = Self::random(n_inputs, 1, 2 * id_leave, rng, with_shift);
            leave.data = NodeContent::UnaryNode {
                op: rng.random(),
                child: Box::from(child),
            };
        } else {
            let left_child = Self::random(n_inputs, 1, 2 * id_leave, rng, with_shift);
            let right_child = Self::random(n_inputs, 1, 2 * id_leave + 1, rng, with_shift);
            leave.data = NodeContent::BinaryNode {
                op: rng.random(),
                left: Box::from(left_child),
                right: Box::from(right_child),
            };
        }
    }

    fn choose_leave(&self, equilibrate: bool, rng: &mut impl Rng) -> usize {
        if equilibrate {
            match &self.data {
                NodeContent::Input(_) => self.id.into(),
                NodeContent::UnaryNode { op: _, child } => child.choose_leave(equilibrate, rng),
                NodeContent::BinaryNode { op: _, left, right } => {
                    let choose_left = rng.random_bool(0.5);
                    if choose_left {
                        left.choose_leave(equilibrate, rng)
                    }
                    else {
                        right.choose_leave(equilibrate, rng)
                    }
                },
            }
        }
        else {
            let leaves = self.get_leaves();
            let index = rng.random_range(0..leaves.len());
            leaves[index]
        }
    }

    fn get_leaves(&self) -> Vec<usize> {
        match &self.data {
            NodeContent::Input(_) => {
                let mut result = Vec::new();
                result.push(self.id.into());
                result
            },
            NodeContent::UnaryNode { op: _, child } => child.get_leaves(),
            NodeContent::BinaryNode { op: _, left, right } => {
                let left_leaves = left.get_leaves();
                let right_leaves = right.get_leaves();
                [left_leaves, right_leaves].concat()
            },
        }
    }

    fn _find_gate(&mut self, id: usize) -> &mut Self {
        let moves = moves_for_id(id);
        self._walk(moves)
    }

    fn _walk(&mut self, mut moves: Vec<Move>) -> &mut Self {
        if let Some(m) = moves.pop() {
            self.data._walk(m, moves)
        } else {
            self
        }
    }

    fn walk_outputs(&mut self, mut moves: Vec<Move>) -> &mut Self {
        // effectue la même chose que walk en retirant les valeurs des outputs sur son passage
        self.outputs = None;
        if let Some(m) = moves.pop() {
            self.data.walk_outputs(m, moves)
        } else {
            self
        }
    }

    pub fn update_gate(&mut self, id: usize, op: Op) {
        let moves = moves_for_id(id);
        let gate = self.walk_outputs(moves);
        assert_eq!(gate.id.get(), id);
        gate.data.set_op(op);
        self.clear_backward();
    }

    fn compute_deltas(&mut self, inputs: &[Vec<u32>], with_shift: bool) -> Deltas {
        self.data
            .compute_deltas(self.grads.as_ref().unwrap(), self.outputs.as_ref().unwrap(), inputs, with_shift)
    }

    fn compute_scores(&mut self, inputs: &[Vec<u32>], scores: &mut Scores, with_shift: bool) {
        scores.values.insert(self.id.into(), self.compute_deltas(inputs, with_shift));
        self.data.compute_scores(inputs, scores, with_shift);
    }

    pub fn get_scores(&mut self, instance: &Instance, with_shift: bool) -> Scores {
        let inputs = instance.inputs.as_slice();
        let targets = &instance.outputs;

        let init_grads = targets.iter().copied().map(|y| Grad {influence: 0u32.not(), target: y}).collect();
        self.forward(inputs);
        self.backward(init_grads);
        let mut scores = Scores::new();
        self.compute_scores(inputs, &mut scores, with_shift);
        scores
    }

    pub fn current_score(&mut self, instance: &Instance, n_examples: usize) -> f32 {
        let inputs = instance.inputs.as_slice();
        let targets = &instance.outputs;

        let p = |n: u32| {u32::count_ones(n) as f32};
        let outputs = self.forward(inputs);
        let s: f32 = outputs.iter().zip(targets).map(|(x, y)| {
                            p((x.bitxor(y)).not())
                        }).sum();
        s/(32.*f32::value_from(n_examples).unwrap())
    }

    pub fn size(&self) -> usize {
        self.data.size()
    }

    pub fn compare(&mut self, n_inputs: usize, other: &mut Node, rng: &mut impl Rng) -> f32 {
        let n_examples = 1000;
        let instance = other.to_instance(n_inputs, n_examples, rng);
        self.clear_forward();
        let s = self.current_score(&instance, n_examples);
        self.clear_forward();
        s
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Input(usize),
    Unary(UnaryOp),
    Binary(BinaryOp),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Left,
    Right,
}

fn moves_for_id(mut id: usize) -> Vec<Move> {
    assert!(id > 0, "Node ids are non-zero integers");
    let mut moves = vec![];
    while id > 1 {
        moves.push(if id & 1 == 0 { Move::Left } else { Move::Right });
        id >>= 1;
    }
    moves
}

impl Display for NodeContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeContent::Input(i) => write!(f, "x_{i}"),
            NodeContent::UnaryNode { op, child } => write!(f, "({child}) {op}"),
            NodeContent::BinaryNode { op, left, right } => write!(f, "({left}) {op} ({right})"),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[cfg(test)]
mod test {
    use rand::{Rng, rng};

    use crate::{formula::{grad::Grad, Node}, solving::Instance};

    #[test]
    fn random_size() {
        for _ in 0..100 {
            let n_inputs: usize = rng().random_range(1..1000);
            let size: usize = rng().random_range(1..1000);
            let f = Node::random(n_inputs, size, 1, &mut rng(), true);
            assert_eq!(f.size(), size)
        }
    }

    #[test]
    fn from_str_size() {
        let s = "((((x_0) << 19) >> 26) & ((x_0) ^ (x_0))) ^ (((((x_0) << 11) & (((x_0) !) & (x_0))) ^ (x_0)) ^ (((x_0) & ((x_0) << 1)) & (((x_0) << 17) >> 5)))";
        let f = Node::from_str(s, 1);
        assert_eq!(f.size(), 26)
    }

    #[test]
    fn to_string_from_str() {
        for _ in 0..100 {
            let n_inputs: usize = rng().random_range(1..1000);
            let size: usize = rng().random_range(1..1000);
            let f = Node::random(n_inputs, size, 1, &mut rng(), true);
            let s = f.to_string();
            let new_f = Node::from_str(&s, 1);
            assert_eq!(f.size(), new_f.size());
            assert_eq!(s, new_f.to_string())
        }
    }

    #[test]
    fn forward() {
        let s = "(((x_0) & (x_1)) !) ^ ((x_0) >> 3)";
        let mut input= Vec::new();
        input.push(25);
        input.push(21);
        let inputs = [input];
        let mut f = Node::from_str(s, 1);
        let result = f.forward(inputs.as_slice());
        assert_eq!(result[0], u32::max_value()-18);
    }

    #[test]
    fn backward() {
        let s = "(((x_0) & (x_1)) !) ^ ((x_0) >> 3)";
        let y = u32::max_value()-27;
        let init_grads = Box::new([Grad {influence: u32::max_value(), target: y}]);
        let mut input= Vec::new();
        input.push(25);
        input.push(21);
        let inputs = [input];
        let mut f = Node::from_str(s, 1);
        f.forward(inputs.as_slice());
        f.backward(init_grads);
        let max = u32::max_value();
        let id_influence_target = [
            (1, max, max-27), 
            (2, max, max-24), 
            (4, max, 24), 
            (8, 21, 24), 
            (9, 25, 24), 
            (3, max, 10), 
            (6, max-7, 80)];
        for (id, exp_influence, exp_target) in id_influence_target {
            let g = f._find_gate(id);
            assert!(g.grads.is_some());
            let Grad { influence, target } = g.grads.clone().unwrap()[0];
            assert_eq!(exp_influence, influence);
            assert_eq!(exp_target & influence, target & influence);
        }
    }
}


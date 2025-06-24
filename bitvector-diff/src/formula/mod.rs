use rand::Rng;

use crate::formula::{binary::BinaryOp, grad::Grad, unary::UnaryOp};

mod binary;
mod grad;
mod unary;

#[derive(Debug)]
pub struct Node {
    id: usize,
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
    pub fn forward(&mut self, inputs: &[&[u32]]) -> Box<[u32]> {
        match self {
            NodeContent::Input(i) => inputs[*i].iter().copied().collect(),
            NodeContent::UnaryNode { op, child } => {
                let child_out = child.forward(inputs);
                child_out.iter().map(|x| op.apply(*x)).collect()
            }
            NodeContent::BinaryNode { op, left, right } => {
                let left_out = left.forward(inputs);
                let right_out = right.forward(inputs);
                left_out
                    .into_iter()
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
}

impl Node {
    fn new(id: usize, data: NodeContent) -> Self {
        Self {
            id,
            data,
            outputs: None,
            grads: None,
        }
    }
    pub fn forward<'s, 'i>(&'s mut self, inputs: &'i [&'i [u32]]) -> &'s [u32] {
        self.outputs
            .get_or_insert_with(|| self.data.forward(inputs))
    }

    pub fn backward(&mut self, grads: Box<[Grad]>) {
        self.grads.get_or_insert_with(|| {
            self.data.backward(&grads);
            grads
        });
    }

    pub fn random(n_inputs: usize, size: usize, id: usize, rng: &mut impl Rng) -> Self {
        if size == 1 {
            Node::new(id, NodeContent::Input(rng.random_range(0..n_inputs)))
        } else {
            let is_unary = rng.random_bool(0.25) || size == 2; // TODO: change proba ?
            if is_unary {
                let child = Self::random(n_inputs, size - 1, id + 1, rng);
                let data = NodeContent::UnaryNode {
                    op: rng.random(),
                    child: Box::from(child),
                };
                Node::new(id, data)
            } else {
                let left_size = rng.random_range(1..=(size - 2));
                let left_child = Self::random(n_inputs, left_size, 2 * id + 1, rng);
                let right_size = size - 1 - left_size;
                let right_child = Self::random(n_inputs, right_size, 2 * id + 2, rng);
                let data = NodeContent::BinaryNode {
                    op: rng.random(),
                    left: Box::from(left_child),
                    right: Box::from(right_child),
                };
                Node::new(id, data)
            }
        }
    }
}

use bitvector_diff::formula::Node;
use rand::rng;

fn main() {
    let mut rng = rng();
    let f = Node::random(5, 8, 1, &mut rng);
    println!("{f:?}");
    println!("{f:}");
}

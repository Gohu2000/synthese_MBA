use bitvector_diff::formula::Node;
use rand::rng;

fn main() {
    let b= 32f32;
    let (raw_n, raw_m, raw_tau): (u16, u16, f32) = (32, 5, 20.0);
    let (n, m) = (usize::from(raw_n), usize::from(raw_m));
    let tau = raw_tau/(f32::from(raw_n)*b);

    let mut rng = rng();
    let mut f = Node::random(m, 8, 1, &mut rng);
    let (inputs, outputs) = f.to_instance(m, n, &mut rng);
    let mut g = Node::random(m, 8, 1, &mut rng);
    let scores = g.get_scores(inputs.as_slice(), &outputs);
    let (id, op) = scores.softmax(tau);
    println!("{f:}");
    println!("{g:}");
    g.update_gate(id, op);
    println!("{g:}");
}

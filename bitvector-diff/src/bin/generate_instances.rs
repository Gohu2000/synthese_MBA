use bitvector_diff::solving::{JsonData, Parametres};
use rand::{rng, rngs::ThreadRng, SeedableRng};
use rand_chacha::ChaCha20Rng;

fn generate_instances(n: usize, folder: &str) {
    let params: Parametres = (5, 128, 10);
    let mut rng: ChaCha20Rng = SeedableRng::seed_from_u64(42);
    for i in 0..n {
        let json_data = JsonData::random(params, &mut rng);
        json_data.to_file(format!("{folder}/instance_{i}.json").as_str());
    }
}

fn main() {
    generate_instances(50, "instances")
}
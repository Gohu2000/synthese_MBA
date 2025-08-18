use bitvector_diff::{solving::{json_data::{JsonData, XyntiaJsonData}, Parametres}};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

fn usize_to_string(n: usize, nb_chiffres: usize) -> String {
    let string = format!("{n}");
    let m = nb_chiffres - string.len();
    let mut buffer = String::new();
    for _ in 0..m {
        buffer.push('0');
    }
    buffer.push_str(&string);
    buffer
}

fn generate_instances(n: usize, folder: &str) {
    let mut rng: ChaCha20Rng = SeedableRng::seed_from_u64(42);
    for size in [4, 8, 12, 16, 20, 24] {
        for n_inputs in [2, 4, 8, 12, 16, 20, 25, 30] {
            for n_examples in [64, 128, 256, 512] {   
                for i in 0..n {
                    let params = (n_inputs, n_examples, size);
                    let json_data = JsonData::random(params, &mut rng, false);
                    let xyntia_json_data = XyntiaJsonData::from(&json_data.instance);
                    json_data.to_file(format!("{folder}/instance_{}_{}_{}_{i}.json",
                                                        usize_to_string(n_inputs, 2),
                                                        usize_to_string(n_examples, 3),
                                                        usize_to_string(size, 2)).as_str());
                    xyntia_json_data.to_file(format!("xyntia_{folder}/instance_{}_{}_{}_{i}.json",
                                                        usize_to_string(n_inputs, 2),
                                                        usize_to_string(n_examples, 3),
                                                        usize_to_string(size, 2)).as_str());
                }
            }
        }
    }
}

fn old_generate_instances(n: usize, folder: &str) {
    let params: Parametres = (5, 128, 10);
    let mut rng: ChaCha20Rng = SeedableRng::seed_from_u64(42);
    for i in 0..n {
        let json_data = JsonData::random(params, &mut rng, true);
        json_data.to_file(format!("{folder}/instance_{i}.json").as_str());
    }
}

fn main() {
    generate_instances(100, "instances_no_shift")
}
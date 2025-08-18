from glob import glob

folder = "/home/gbathie/synthese_MBA/bitvector-diff/xyntia_instances"
timeout = 60

for i in glob(folder + "/*.json"):
    print(f"timeout {timeout} /home/gbathie/synthese_MBA/bitvector-diff/target/release/bitvector-diff {i}")

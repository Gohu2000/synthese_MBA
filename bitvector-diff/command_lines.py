from glob import glob

folder = "~/synthese_MBA/bitvector-diff/instances"
timeout = 60

for i in glob(folder + "/*.json"):
    print(f"timeout {timeout} ~/synthese_MBA/bitvector-diff/target/release/bitvector-diff {i}")

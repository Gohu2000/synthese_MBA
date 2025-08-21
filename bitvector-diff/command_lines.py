from glob import glob

folder = "/home/hbarzu/synthese_MBA/bitvector-diff/instances_no_shift"
timeout = 60

files = glob(folder + "/instance_??_512_??_*.json")
for i in files:
    print(f"timeout {timeout} ~/synthese_MBA/bitvector-diff/target/release/bitvector-diff {i}")

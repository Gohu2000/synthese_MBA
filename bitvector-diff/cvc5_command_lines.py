from glob import glob

folder = "/home/gbathie/synthese_MBA/bitvector-diff/instances"
timeout = 60

for i in glob(folder + "/*.json"):
    print(f"timeout {timeout} python3 /home/gbathie/synthese_MBA/sat_solver.py {i}")

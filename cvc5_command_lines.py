from glob import glob

folder = "~/synthese_MBA/bitvector-diff/instances"
timeout = 60

for i in glob(folder + "/*.json"):
    print(
        f"source ~/synthese_MBA/.venv/bin/activate && timeout {timeout} python3 ~/synthese_MBA/sat_solver.py {i}"
    )

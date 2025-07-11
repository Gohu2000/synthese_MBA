import argparse
from glob import glob

folder = "instances"
timeout = 60

for i in glob(folder + "/*.json"):
    print(f"timeout {timeout} ./target/release/bitvector-diff {i}")
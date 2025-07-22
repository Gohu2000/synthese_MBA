#!/bin/bash

cd /home/gbathie/synthese_MBA/bitvector-diff
cargo build -r

source cvc5_env/bin/activate
python3 cvc5_command_lines.py | parallel -j25 --joblog ./parallel.log --ssh oarsh --slf $OAR_NODEFILE
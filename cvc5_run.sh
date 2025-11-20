#!/bin/bash

cd ~/synthese_MBA/
source .venv/bin/activate


python3 cvc5_command_lines.py | parallel -j25 --joblog ./parallel.log --ssh oarsh --slf $OAR_NODEFILE
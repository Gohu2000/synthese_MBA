#!/bin/bash

cd ~/synthese_MBA/bitvector-diff

python3 command_lines.py | parallel -j25 --joblog ./parallel.log --ssh oarsh --slf $OAR_NODEFILE
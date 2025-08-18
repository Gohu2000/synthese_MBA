#!/bin/bash

cd ~/synthese_MBA/


python3 xyntia_command_lines.py | parallel -j25 --joblog ./parallel.log --ssh oarsh --slf $OAR_NODEFILE
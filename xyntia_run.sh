#!/bin/bash

set -e

cd ~/synthese_MBA/xyntia
{
sudo-g5k apt update
sudo-g5k apt install -y libgmp3-dev gcc-multilib gdb python3 python3-pip python3-venv openjdk-17-jdk libgmp-dev pkg-config opam

opam init -y --disable-sandboxing
eval $(opam env)
opam switch create . 4.14.1 -y
eval $(opam env)
} > /dev/null 2>&1

python3 xyntia_command_lines.py | parallel -j25 --joblog ./parallel.log --ssh oarsh --slf $OAR_NODEFILE
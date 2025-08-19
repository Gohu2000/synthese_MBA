cd ~/synthese_MBA/bitvector-diff
cargo build -r

oarsub -l host=1,walltime=1:00:00 -p "cluster='paradoxe'" ~/synthese_MBA/bitvector-diff/target/release/generate_instances -q default
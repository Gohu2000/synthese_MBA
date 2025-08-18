oarsub -I -l host=1,walltime=1:00:00 -p "cluster='paradoxe'"
cd ~/synthese_MBA/xyntia
opam switch create . 4.14.1 -y
eval $(opam env)
sudo-g5k apt install libgmp3-dev gcc-multilib gdb python3 python3-pip python3-venv openjdk-17-jdk libgmp-dev pkg-config opam
eval $(oarsub -l host=1,walltime=1:00:00 -p "cluster='paradoxe'" ~/synthese_MBA/xyntia_run.sh -q default)
echo OAR.${OAR_JOB_ID}.stdout
echo OAR.${OAR_JOB_ID}.stderr
echo scp -r rennes.g5k:~/synthese_MBA/OAR.${OAR_JOB_ID}.stdout ./stdout
watch wc -l ~/synthese_MBA/xyntia/OAR.${OAR_JOB_ID}.stdout

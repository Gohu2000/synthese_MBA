eval $(oarsub -l host=1,walltime=1:00:00 -p "cluster='paradoxe'" ~/synthese_MBA/cvc5_run.sh -q default)
echo OAR.${OAR_JOB_ID}.stdout
echo OAR.${OAR_JOB_ID}.stderr
echo scp -r rennes.g5k:~/synthese_MBA/OAR.${OAR_JOB_ID}.stdout ./stdout
watch wc -l OAR.${OAR_JOB_ID}.stdout

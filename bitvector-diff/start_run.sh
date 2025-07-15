eval $(oarsub -l host=1,walltime=1:00:00 -p "cluster='paradoxe'" /home/gbathie/synthese_MBA/bitvector-diff/run.sh -q default)
echo OAR.${OAR_JOB_ID}.stdout
echo OAR.${OAR_JOB_ID}.stderr
watch wc -l OAR.${OAR_JOB_ID}.stdout
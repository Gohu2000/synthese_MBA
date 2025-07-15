oar_job_id = input().split("=")[1]
print("watch wc -l OAR." + oar_job_id + ".stdout")
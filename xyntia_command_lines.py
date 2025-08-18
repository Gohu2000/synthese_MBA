from glob import glob

folder = "~/synthese_MBA/bitvector-diff/xyntia_instances_no_shift"
timeout = 60

for i in glob(folder + "/*.json"):
    print(f"cd xyntia && echo {i} && xyntia -ops 'not,&,^,|' {i} | python3 ~/synthese_MBA/xyntia_output.py")

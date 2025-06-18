from examples import*

def result_example_file(algo, result_file, filename, nb_instances, dir_benchmark):

    def padding(n, type):
        taille_num_exemple = 3
        taille_score = 3
        if type == "num_exemple":
            s = str(n)
            pad = taille_num_exemple - len(s)
            return s + " "*pad
        if type == "score":
            score = int(n*100)
            s = str(score)
            pad = taille_score - len(s)
            return s + " "*pad

    s = "------------ RESULTS ------------\n\n"
    with open(result_file, "w") as file:
        file.write(s)
        file.write(filename+"\n\n")
        num_exemple = 0
        params_str = (filename.split(".")[0]).split("_")
        n, nb_bits, size = [int(param) for param in params_str]

        for formula, instance in extract_examples(dir_benchmark + "/" + filename):
            print(num_exemple)
            score = algo(instance, n, size, nb_bits)
            file.write(padding(num_exemple, "num_exemple") + " : " + padding(score, "score") + " | ")
            num_exemple += 1
            if num_exemple % nb_instances == 0:
                file.write("\n")
        file.write("\n")


def result_benchmark(algo, result_file, nb_instances, dir_benchmark):

    def padding(n, type):
        taille_num_exemple = 3
        taille_score = 3
        if type == "num_exemple":
            s = str(n)
            pad = taille_num_exemple - len(s)
            return s + " "*pad
        if type == "score":
            score = int(n*100)
            s = str(score)
            pad = taille_score - len(s)
            return s + " "*pad

    s = "------------ RESULTS ------------\n\n"
    with open(result_file, "w") as file:
        files = os.listdir(dir_benchmark)
        files.sort()
        file.write(s)
        for filename in files:
            file.write(filename+"\n\n")
            print(filename)
            num_exemple = 0
            params_str = (filename.split(".")[0]).split("_")
            n, nb_bits, size = [int(param) for param in params_str]

            for formula, instance in extract_examples(f"{dir_benchmark}/" + filename):
                score = algo(instance, n, size, nb_bits)
                file.write(padding(num_exemple, "num_exemple") + " : " + padding(score, "score") + " | ")
                num_exemple += 1
                if num_exemple % nb_instances == 0:
                    file.write("\n")
            file.write("\n")

def exploit_results(f_exploit, l_nb_inputs, result_file, exploited_result_file):
    dico_results = {}
    
    with open(result_file, "r") as file:
        lines = file.readlines()
        i=2
        data_filename = lines[i][:-1]
        params_str = (data_filename.split(".")[0]).split("_")
        n, nb_bits, size = [int(param) for param in params_str]
        i+=2
        line = lines[i]
        dico_results[(n, nb_bits, size)] = {}
        j=0
        while line != "\n":
            scores = [int(example.split(":")[1]) for example in line[:-1].split("|")[:-1]]
            mean = int(sum(scores)/len(scores))
            dico_results[(n, nb_bits, size)][l_nb_inputs[j]] = scores
            i+=1
            line = lines[i]
        i+=1

    f_exploit(result_file, exploited_result_file, dico_results)
def basic_exploit(result_file, exploited_result_file, dico_results):
    with open(exploited_result_file, "a") as file:
        params= list(dico_results.keys())
        params.sort()
        for n, nb_bits, size in params:
            sub_dico = dico_results[(n, nb_bits, size)]
            l_nb_inputs = list(sub_dico.keys())
            l_nb_inputs.sort()
            for nb_input in l_nb_inputs:
                scores = sub_dico[nb_input]
                file.write(result_file + "\n")
                mean = round(sum(scores)/len(scores), 2)
                nb_succes = 0
                for score in scores:
                    if score == 100:
                        nb_succes += 1
                file.write(f"moyenne : {mean}\n")
                file.write(f"nombre de succès : {nb_succes} sur {len(scores)}\n\n")


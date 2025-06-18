from bitvector_tools import*
from init_functions import*
import os

default_params = [(5, 15, 16, 5) for i in range(10)]
default_tau = 10
default_example_file = "examples/examples.txt"

def generate_examples(filename = default_example_file, params = default_params):
    s = "------------- EXEMPLES BITVECTORS -------------\n\n"
    s += "Nombre d'exemples :\n"
    s += str(len(params)) + "\n"
    s += "Paramètres utilisés :\n"
    s += str(params) + "\n\n"
    i = 0
    for n, size, nb_inputs, nb_bits in params:
        node = create_rd_formula(n, size, nb_bits)
        f_seed = node.get_seed()
        instance = derive_instance(n, nb_inputs, node, nb_bits)
        i_seed = instance_to_seed(instance)
        s += f"Exemple {i}:\n"
        s += "Nombre de variables | " + str(n) + "\n"
        s += "Nombre de bits      | " + str(nb_bits) + "\n"
        s += "Seed de la formule  | " + f_seed + "\n"
        s += "Seed de l'instance  | " + i_seed + "\n\n"
        i += 1

    with open(filename, "w") as file:
        file.write(s)

def extract_examples(filename = default_example_file):
    with open(filename, "r") as file:
        lines = file.readlines()

    nb_examples = int(lines[3])
    i = 7
    l_examples = []
    for j in range(nb_examples):
        i += 2
        nb_bits = int(lines[i].split("| ")[1][:-1])
        i += 1
        f_seed = lines[i].split("| ")[1][:-1]
        i += 1
        i_seed = lines[i].split("| ")[1][:-1]
        i += 2
        l_examples.append((recreate_formula(f_seed), seed_to_instance(nb_bits, i_seed)))
    return l_examples

def print_example(filename, formula, instance, tau = default_tau, num_input=0):
    current_score = formula.score_formula(instance)
    formula.change_tau(tau)
    formula.update_tree(instance)
    nb_bits = instance[0]
    nb_vars = len(instance[1][0])

    x = formula.calcul(instance[num_input+1][0], nb_bits)
    value = formula.image()
    score = formula.image("score")
    proba = formula.image("proba")
    result = formula.image("result")

    s = f"Tau : {tau}\n"
    s += f"Nombre de bits : {nb_bits}\n"
    s += f"Nombre de variables : {nb_vars}\n"
    s += f"Score actuel : {current_score}\n"
    s += "Instance :\n" + str(instance) + "\n\n"
    s += "Formule :\n\n"
    s += value + "\n"
    s += f"Résultat pour l'input {num_input} :\n\n"
    s += result + "\n"
    s += "Score :\n\n"
    s += score + "\n"
    s += "Proba softmax :\n\n"
    s += proba + "\n"

    with open(filename, "w") as file:
        file.write(s)

def f_inputs_max_to_l_nb_input(f_inputs_max, n, nb_bits, inputs_factor):
    nb_inputs = 1
    l_nb_inputs = []
    while nb_inputs <= f_inputs_max(n, nb_bits):
        if int(nb_inputs) not in l_nb_inputs:
            l_nb_inputs.append(int(nb_inputs))
        nb_inputs = nb_inputs*inputs_factor
    return l_nb_inputs

def generate_benchmark(l_n, l_size, l_nb_bits, l_nb_inputs, nb_instances, dir_benchmark):
    for file in os.listdir(dir_benchmark):
        os.remove(dir_benchmark + "/" + file)
    for n in l_n:
        for nb_bits in l_nb_bits:
            for size in l_size:
                params = []
                for nb_inputs in l_nb_inputs:
                    params += [(n, size, nb_inputs, nb_bits) for i in range(nb_instances)]

                generate_examples(f"{dir_benchmark}/{n}_{nb_bits}_{size}.txt", params)

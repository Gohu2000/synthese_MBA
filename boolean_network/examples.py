from boolean_tools import*
from init_functions import*

default_params = [(5, 15, 16) for i in range(10)]
default_tau = 10
default_example_file = "examples.txt"

def generate_examples(filename = default_example_file, params = default_params):
    s = "------------- EXEMPLES BOOLEENS -------------\n\n"
    s += "Nombre d'exemples :\n"
    s += str(len(params)) + "\n"
    s += "Paramètres utilisés :\n"
    s += str(params) + "\n\n"
    i = 0
    for n, size, nb_inputs in params:
        node = create_rd_formula(n, size)
        f_seed = node.get_seed()
        instance = derive_instance(n, nb_inputs, node)
        i_seed = instance_to_seed(instance)
        s += f"Exemple {i}:\n"
        s += "Nombre de bits     | " + str(n) + "\n"
        s += "Seed de la formule | " + f_seed + "\n"
        s += "Seed de l'instance | " + i_seed + "\n\n"
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
        i += 1
        n = int(lines[i].split("| ")[1][:-1])
        i += 1
        f_seed = lines[i].split("| ")[1][:-1]
        i += 1
        i_seed = lines[i].split("| ")[1][:-1]
        i += 2
        l_examples.append((recreate_formula(f_seed), seed_to_instance(n, i_seed)))
    return l_examples

def print_example(filename, formula, instance, tau = default_tau):
    current_score = formula.score_formula(instance)
    formula.change_tau(tau)
    formula.update_tree(instance)
    value = formula.image()
    score = formula.image("score")
    proba = formula.image("proba")
    nb_bits = len(instance[0][0])

    s = f"Tau : {tau}\n"
    s += f"Nombre de bits : {nb_bits}\n"
    s += f"Score actuel : {current_score}\n"
    s += "Instance :\n" + str(instance) + "\n\n"
    s += "Formule :\n\n"
    s += value + "\n"
    s += "Score :\n\n"
    s += score + "\n"
    s += "Proba softmax :\n\n"
    s += proba + "\n"

    with open(filename, "w") as file:
        file.write(s)
import random
from math import exp
from tools import *

dico_operators = {0: "and", 1: "or", 2: "not", 3: "id"}
gates = {1: ["not", "id"], 2: ["and", "or"]}
default_params = [(5, 15, 16) for i in range(10)]
default_tau = 10


class Node:
    def __init__(self, value=None):
        self.value = value
        self.parent = None
        self.children = []
        self.score = {}
        self.proba = {}

    def add_child(self, child):
        assert child.parent == None
        self.children.append(child)
        child.parent = self

    def image(self, param="value", prefixe="", dernier=True):
        node_content = str(self.value)
        if param == "score":
            node_content = str(self.score)
        if param == "proba":
            node_content = str(self.proba)
        branche = "└── " if dernier else "├── "
        image = prefixe + branche + node_content + "\n"
        prefixe += "    " if dernier else "│   "
        for i, child in enumerate(self.children):
            last_child = i == len(self.children) - 1
            image += child.image(param, prefixe, last_child)
        return image

    def print(self, param="value", prefixe="", dernier=True):
        node_content = str(self.value)
        if param == "score":
            node_content = str(self.score)
        if param == "proba":
            node_content = str(self.proba)
        branche = "└── " if dernier else "├── "
        print(prefixe + branche + node_content)
        prefixe += "    " if dernier else "│   "
        for i, child in enumerate(self.children):
            last_child = i == len(self.children) - 1
            child.print(param, prefixe, last_child)

    def update_tree_score(self, instance):
        self.update_score(instance, self, score_formula(self, instance))

    def update_score(self, instance, root, score):
        arity = len(self.children)
        n = len(instance[0][0])
        current_value = self.value
        self.score = {}
        l_gates = list(range(n))
        if arity != 0:
            l_gates = gates[arity]

        for i in l_gates:
            if i != current_value:
                self.value = i
                self.score[i] = score_formula(root, instance) - score
        self.value = current_value

        for child in self.children:
            child.update_score(instance, root, score)

    def update_tree_proba_softmax(self, tau=default_tau):
        self.update_proba_softmax(self, get_sum_softmax(self, tau), tau)

    def update_proba_softmax(self, root, sum, tau=default_tau):
        f = lambda x: exp(tau * x)
        self.proba = {}
        for gate in self.score:
            self.proba[gate] = round(f(self.score[gate]) / sum, 2)

        for child in self.children:
            child.update_proba_softmax(root, sum, tau)

    def update_tree(self, instance, tau=default_tau):
        self.update_tree_score(instance)
        self.update_tree_proba_softmax(tau)

    def get_seed(self):
        split_c = " "
        seed = str(self.value)

        for child in self.children:
            child_seed = child.get_seed()
            n = len(child_seed)
            seed = seed + split_c + str(n) + split_c + child_seed
        return seed + split_c


def get_sum_softmax(root, tau=default_tau):
    f = lambda x: exp(tau * x)

    def dfs_sum(node):
        node_sum = 0
        for k, v in node.score.items():
            node_sum += f(v)

        for child in node.children:
            node_sum += dfs_sum(child)
        return node_sum

    return dfs_sum(root)


def softmax(root, tau=default_tau):
    f = lambda x: exp(tau * x)

    def dfs_proba(node, previous_proba):
        node_proba = previous_proba
        for k, v in node.score.items():
            node_proba += f(v) / sum
            if p <= node_proba:
                node.value = k
                return -1

        for child in node.children:
            node_proba = dfs_proba(child, node_proba)
            if node_proba == -1:
                return -1

        return node_proba

    sum = get_sum_softmax(root, tau)
    p = random.random()
    x = dfs_proba(root, 0)
    assert x == -1


def create_rd_formula(n, size):
    if size == 1:
        x = random.randint(0, n - 1)
        return Node(x)

    if size == 2:
        node = Node("not")
        x = random.randint(0, n - 1)
        node.add_child(Node(x))
        return node

    left = random.randint(1, size - 2)
    right = size - 1 - left

    node_left = create_rd_formula(n, left)
    node_right = create_rd_formula(n, right)

    nb_gate = len(gates[2])
    i = random.randint(0, nb_gate - 1)
    node = Node(gates[2][i])
    node.add_child(node_left)
    node.add_child(node_right)

    return node


def recreate_formula(seed):
    split_c = " "

    def next_token():
        j = 0
        c = seed[i + j]
        token = ""
        while c != split_c:
            token += c
            j += 1
            c = seed[i + j]
        return j, token

    i = 0
    j, value = next_token()
    i += j + 1
    if seed[i:] == "":
        return Node(int(value))

    node = Node(value)

    while seed[i:] != "":
        j, size = next_token()
        size = int(size)
        i += j + 1
        child = recreate_formula(seed[i : i + size])
        node.add_child(child)
        i += size + 1
    return node


def derive_instance(n, nb_examples, formula):
    assert nb_examples <= 2**n
    instance = []
    l_input = []
    input = [0 for i in range(n)]
    for i in range(2**n - 1):
        input = input.copy()
        l_input.append(input)
        j = 0
        while input[j] == 1:
            input[j] = 0
            j += 1
        input[j] = 1
    l_input.append(input)
    for i in range(nb_examples):
        n_input = random.randint(0, 2**n - 1 - i)
        input = l_input.pop(n_input)
        y = calcul(formula, input)
        instance.append((input, y))

    return instance


def create_rd_instance(n, size, nb_examples):
    formula = create_rd_formula(n, size)
    return derive_instance(n, nb_examples, formula), formula


def calcul(root, input):
    n = len(root.children)
    assert n in [0, 1, 2]
    if n == 0:
        return input[root.value]
    if n == 1:
        assert root.value in gates[1]
        child = root.children[0]
        if root.value == "not":
            return 1 - calcul(child, input)
        if root.value == "id":
            return calcul(child, input)
    if n == 2:
        child1 = root.children[0]
        child2 = root.children[1]
        assert root.value in gates[2]
        if root.value == "or":
            return calcul(child1, input) or calcul(child2, input)
        if root.value == "and":
            return calcul(child1, input) and calcul(child2, input)


def score_formula(f, instance):
    score = 0
    n = len(instance)
    for input, y in instance:
        result = calcul(f, input)
        if result == y:
            score += 1
    return score / n


def performance_formula(f, instance):
    for input, y in instance:
        result = calcul(f, input)
        print("obtenu : ", result, " attendu : ", y)
    print(score_formula(f, instance))


def evolution(f, instance, tau=5, N=10):
    score = 0
    for i in range(N):
        score = score_formula(f, instance)
        print(score)
        if score > 0.99:
            return score
        f.update_tree_score(instance)
        softmax(f, tau)
    return score


def progressive_evolution(f, instance, N, tau_max=10):
    score = 0
    tau = 1
    # (tau_max - tau)/step + 1 = N / 10
    step = 10 * (tau_max - tau) / (N - 10)
    for i in range(N // 10):
        score = evolution(f, instance, tau)
        if score > 0.99:
            return score
        tau += step

    return score


def generate_examples(filename="examples.txt", params=default_params):
    s = "-------------EXEMPLES-------------\n\n"
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


def extract_examples(filename="examples.txt"):
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


def print_example(filename, formula, instance, tau=default_tau):
    current_score = score_formula(formula, instance)
    formula.update_tree(instance, tau)
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


def test_evolution(n, size, progressive=True):
    node = create_rd_formula(n, size)
    instance, goal = create_rd_instance(n, size, 2 ** (n - 1))
    score = 0
    if progressive:
        score = progressive_evolution(node, instance, 1000)
    else:
        score = evolution(node, instance, tau=20, N=100)
    goal.print()
    node.print()
    node.print("score")
    print(score)


def test_seed(n, size):
    node = create_rd_formula(n, size)
    seed = node.get_seed()
    node.print()
    copy = recreate_formula(seed)
    copy.print()


def test_examples():
    generate_examples()
    formula, instance = extract_examples()[0]
    print_example("example_0.txt", formula, instance)


test_evolution(5, 15)
# test_evolution(5, 15, False)
# test_seed(5, 15)
# test_examples()

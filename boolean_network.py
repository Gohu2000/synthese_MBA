import random
from math import exp

dico_operators = {0: "and", 1: "or", 2: "not", 3: "id"}
gates = {1: ["not", "id"], 2: ["and", "or"]}

class Node:
    def __init__(self, value = None):
        self.value = value
        self.parent = None
        self.children = []
        self.score = {}
        
    def add_child(self, child):
        assert child.parent == None
        self.children.append(child)
        child.parent = self

    def print(self, prefixe="", dernier=True):
        node_value = str(self.value)
        branche = "└── " if dernier else "├── "
        print(prefixe + branche + node_value)
        prefixe += "    " if dernier else "│   "
        for i, child in enumerate(self.children):
            last_child = (i == len(self.children) - 1)
            child.print(prefixe, last_child)

    def print_score(self, prefixe="", dernier=True):
        node_value = str(self.score)
        branche = "└── " if dernier else "├── "
        print(prefixe + branche + node_value)
        prefixe += "    " if dernier else "│   "
        for i, child in enumerate(self.children):
            last_child = (i == len(self.children) - 1)
            child.print_score(prefixe, last_child)

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

        for child in self.children:
            child.update_score(instance, root, score)

    def copy(self):
        return Node(self.value)

def softmax(root, tau = 1):
    f = lambda x: exp(tau*x)

    def dfs_sum(node):
        node_sum = 0
        for k, v in node.score.items():
            node_sum += f(v)

        for child in node.children:
            node_sum += dfs_sum(child)
        return node_sum
    
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


    
    sum = dfs_sum(root)
    p = random.random()
    x = dfs_proba(root, 0)
    assert x == -1

def create_rd_formula(n, size):

    if size == 1:
        x = random.randint(0, n-1)
        return Node(x)
    
    if size == 2:
        node = Node("not")
        x = random.randint(0, n-1)
        node.add_child(Node(x))
        return node
    
    left = random.randint(1, size-2)
    right = size - 1 - left

    node_left = create_rd_formula(n, left)
    node_right = create_rd_formula(n, right)

    gate = random.choice(gates[2])
    node = Node(gate)
    node.add_child(node_left)
    node.add_child(node_right)

    return node

def create_rd_instance(n, size, nb_examples):
    assert nb_examples <= 2**n
    instance = []
    formula = create_rd_formula(n, size)
    l_input = []
    input = [0 for i in range(n)]
    for i in range(2**n-1):
        input = input.copy()
        l_input.append(input)
        j = 0
        while input[j] == 1:
            input[j] = 0
            j += 1
        input[j] = 1
    l_input.append(input)
    print(l_input)
    for i in range(nb_examples):
        n_input = random.randint(0, 2**n-1-i)
        input = l_input.pop(n_input)
        y = calcul(formula, input)
        instance.append((input, y))

    return instance, formula

def calcul(root, input):
    n = len(root.children)
    assert n in [0, 1, 2]
    if n == 0:
        return input[root.value]
    if n == 1:
        assert root.value in gates[1]
        child = root.children[0]
        if root.value == "not":
            return 1-calcul(child, input)
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

def evolution(f, instance, tau = 5, N = 10):
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
    step = 10*(tau_max - tau)/(N-10)
    for i in range(N//10):
        score = evolution(f, instance, tau)
        if score > 0.99:
            return score
        tau += step

    return score

def test(n, size):
    node = create_rd_formula(n, size)
    instance, goal = create_rd_instance(n, size, 2**(n-1))
    score = progressive_evolution(node, instance, 1000)
    goal.print()
    node.print()
    node.print_score()
    print(score)

test(5, 15)

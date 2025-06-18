import random
from softmax_node import SoftmaxNode as Node
from node import GATES

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

    nb_gate = len(GATES[2])
    i = random.randint(0, nb_gate - 1)
    node = Node(GATES[2][i])
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
        child = recreate_formula(seed[i:i+size])
        node.add_child(child)
        i += size + 1
    return node

def derive_instance(n, nb_examples, formula):
    assert nb_examples <= 2**n
    instance = []
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
    for i in range(nb_examples):
        n_input = random.randint(0, 2**n-1-i)
        input = l_input.pop(n_input)
        y = formula.calcul(input)
        instance.append((input, y))

    return instance

def create_rd_instance(n, size, nb_examples):
    formula = create_rd_formula(n, size)
    return derive_instance(n, nb_examples, formula), formula
import random
from softmax_node import SoftmaxNode as Node
from node import GATES
from bitvector_tools import*

def create_rd_formula(n, size, nb_bits):

    if size == 1:
        x = random.randint(0, n-1)
        return Node(x)
    
    if size == 2:
        nb_gate = len(GATES[1])
        i = random.randint(1, nb_gate - 1)
        gate = GATES[1][i]
        if gate == "<<" or gate == ">>":
            j = random.randint(1, nb_bits)
            gate = gate + str(j)
        x = random.randint(0, n-1)
        node = Node(gate)
        node.add_child(Node(x))
        return node
    
    left = random.randint(1, size-2)
    right = size - 1 - left

    node_left = create_rd_formula(n, left, nb_bits)
    node_right = create_rd_formula(n, right, nb_bits)

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

def derive_instance(n, nb_examples, formula, nb_bits):
    assert nb_examples <= 2**(n*nb_bits)
    instance = []
    for i in range(nb_examples):
        input = [int_to_binary(random.randint(0, 2**nb_bits-1), nb_bits) for i in range(n)]
        y = formula.calcul(input)
        instance.append((input, y))

    return instance

def create_rd_instance(n, size, nb_examples, nb_bits):
    formula = create_rd_formula(n, size, nb_bits)
    return derive_instance(n, nb_examples, formula, nb_bits), formula
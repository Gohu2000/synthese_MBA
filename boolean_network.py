import random

dico_operators = {0: "and", 1: "or", 2: "not"}

class Node:
    def __init__(self, value = None):
        self.value = value
        self.parent = None
        self.children = []
        
    def add_child(self, child):
        assert child.parent == None
        self.children.append(child)
        child.parent = self

    def print(self, prefixe="", dernier=True):
        node_value = str(self.value)
        if self.children != []:
            node_value = dico_operators[self.value]
        branche = "└── " if dernier else "├── "
        print(prefixe + branche + node_value)
        prefixe += "    " if dernier else "│   "
        for i, child in enumerate(self.children):
            last_child = (i == len(self.children) - 1)
            child.print(prefixe, last_child)

def create_rd_instance(n, nb_examples):
    l = []
    for i in range(nb_examples):
        l_i = []
        for j in range(n):
            x = random.getrandbits(1)
            l_i.append(x)
        y = random.getrandbits(1)
        l.append((l_i, y))
    return l

def create_rd_program(n, activate_not = True):
    assert n >= 2
    if n == 2:
        x = random.getrandbits(1)
        node_x = Node(x)
        root = node_x
        leaves = [node_x, node_x]
        if activate_not:
            root_not = random.getrandbits(1)
            child_not1 = random.getrandbits(1)
            child_not2 = random.getrandbits(1)
            if root_not:
                root = Node(2)
                root.add_child(node_x)

            if child_not1:
                child1 = Node(2)
                node_x.add_child(child1)
                leaves[0] = child1
            if child_not2:
                child2 = Node(2)
                node_x.add_child(child2)
                leaves[1] = child2
        return root, leaves

    # n != 2:         
    q, r = divmod(n, 2)
    p, leaves = create_rd_program(q + r, activate_not)

    new_leaves = []
    for i in range(q):
        x = random.getrandbits(1)
        node_x = Node(x)
        leaves[i].add_child(node_x)
        children_x = [node_x, node_x]
        if activate_not:
            child_not1 = random.getrandbits(1)
            child_not2 = random.getrandbits(1)
            if child_not1:
                child1 = Node(2)
                node_x.add_child(child1)
                children_x[0] = child1
            if child_not2:
                child2 = Node(2)
                node_x.add_child(child2)
                children_x[1] = child2
        new_leaves = new_leaves + children_x

    if r == 1:
        new_leaves.append(leaves[q])

    return p, new_leaves

def create_tree(n, not_everywhere = False, not_on_leaves = True):
    flag = not not_everywhere and not_on_leaves
    p, leaves = create_rd_program(n, activate_not = not_everywhere)
    i=0
    for node in leaves:
        branch = node
        if flag:
            x = random.getrandbits(1)
            if x:
                branch = Node(2)
                node.add_child(branch)
        branch.add_child(Node(i))
        i+=1

    return p

def calcul(root, input):
    n = len(root.children)
    assert n in [0, 1, 2]
    if n == 0:
        return input[root.value]
    if n == 1:
        assert root.value == 2
        child = root.children[0]
        return 1-calcul(child, input)
    if n == 2:
        child1 = root.children[0]
        child2 = root.children[1]
        assert root.value == 1 or root.value == 0
        if root.value == 1:
            return calcul(child1, input) or calcul(child2, input)
        if root.value == 0:
            return calcul(child1, input) and calcul(child2, input)

def verify_program(p, instance):
    for input, y in instance:
        result = calcul(p, input)
        print("obtenu : ", result, " attendu : ", y)

def test(n, m):
    instance = create_rd_instance(n, m)
    root = create_tree(n)
    root.print()
    print(instance)
    verify_program(root, instance)

test(4, 2)

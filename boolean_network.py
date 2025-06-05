import random

dico_operators = {0: "and", 1: "or", 2: "not", 3: "id"}

class Node:
    def __init__(self, value = None):
        self.value = value
        self.parent = None
        self.children = []
        
    def add_child(self, child):
        assert child.parent == None
        self.children.append(child)
        child.parent = self

    def print(self, prefixe="", dernier=True, operator=False):
        node_value = str(self.value)
        if self.children != [] and operator:
            node_value = dico_operators[self.value]
        branche = "└── " if dernier else "├── "
        print(prefixe + branche + node_value)
        prefixe += "    " if dernier else "│   "
        for i, child in enumerate(self.children):
            last_child = (i == len(self.children) - 1)
            child.print(prefixe, last_child)
    
    def copy(self):
        return Node(self.value)

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
        assert root.value == 2 or root.value == 3
        child = root.children[0]
        if root.value == 2:
            return 1-calcul(child, input)
        if root.value == 3:
            return calcul(child, input)
    if n == 2:
        child1 = root.children[0]
        child2 = root.children[1]
        assert root.value == 1 or root.value == 0
        if root.value == 1:
            return calcul(child1, input) or calcul(child2, input)
        if root.value == 0:
            return calcul(child1, input) and calcul(child2, input)

def create_score_tree(p, instance):
    score = score_program(p, instance)
    l_max_score = []

    def calcul_score(node):
        value = node.value
        if value in [0, 1]:
            node.value = 1 - value
            new_score = score_program(p, instance)
        if value in [2, 3]:
            node.value = 5 - value
            new_score = score_program(p, instance)
        node.value = value
        return new_score

    def dfs(node, new_parent, max_score):
        if node.children == []:
            new_parent.add_child(node.copy())
            return max_score
        else:
            score = calcul_score(node)
            new_node = Node(score)
            new_parent.add_child(new_node)
            new_max_score = max(max_score, score)
            for child in node.children:
                dfs(child, new_node)

    max_score = calcul_score(p)
    root = Node(max_score)
    l_max_score.append(root)
    for child in p.children:
        max_score = dfs(child, root, max_score)

    return root, l_max_score

def score_program(p, instance):
    score = 0
    n = len(instance)
    for input, y in instance:
        result = calcul(p, input)
        if result == y:
            score += 1
    return score / n

def performance_program(p, instance):
    for input, y in instance:
        result = calcul(p, input)
        print("obtenu : ", result, " attendu : ", y)
    print(score_program(p, instance))

def test(n, m):
    instance = create_rd_instance(n, m)
    root = create_tree(n)
    root.print(operator=True)
    score_tree = create_score_tree(root, instance)
    print(instance)
    performance_program(root, instance)
    score_tree.print()

test(20, 10)

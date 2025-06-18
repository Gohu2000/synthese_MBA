GATES = {1: ["not", "id"], 2: ["and", "or"]}

class Node:
    def __init__(self, value = None):
        self.value = value
        self.parent = None
        self.children = []
        self.score = {}
        self.proba = {}
        
    def add_child(self, child):
        assert child.parent == None
        self.children.append(child)
        child.parent = self

    def image(self, param = "value", prefixe="", dernier=True):
        node_content = str(self.value)
        if param == "score":
            node_content = str(self.score)
        if param == "proba":
            node_content = str(self.proba)
        branche = "└── " if dernier else "├── "
        image = prefixe + branche + node_content + "\n"
        prefixe += "    " if dernier else "│   "
        for i, child in enumerate(self.children):
            last_child = (i == len(self.children) - 1)
            image += child.image(param, prefixe, last_child)
        return image

    def print(self, param = "value", prefixe="", dernier=True):
        image = self.image(param, prefixe, dernier)
        print(image)

    def update_tree_score(self, instance):
        pass

    def update_tree_proba(self):
        pass
    
    def update_tree(self, instance):
        self.update_tree_score(instance)
        self.update_tree_proba()

    def get_seed(self, split_c = " "):
        seed = str(self.value)

        for child in self.children:
            child_seed = child.get_seed(split_c)
            n = len(child_seed)
            seed = seed + split_c + str(n) + split_c + child_seed
        return seed + split_c
    
    def calcul(self, input):
        n = len(self.children)
        assert n in [0, 1, 2]
        if n == 0:
            return input[self.value]
        if n == 1:
            assert self.value in GATES[1]
            child = self.children[0]
            if self.value == "not":
                return 1-child.calcul(input)
            if self.value == "id":
                return child.calcul(input)
        if n == 2:
            child1 = self.children[0]
            child2 = self.children[1]
            assert self.value in GATES[2]
            if self.value == "or":
                return child1.calcul(input) or child2.calcul(input)
            if self.value == "and":
                return child1.calcul(input) and child2.calcul(input)
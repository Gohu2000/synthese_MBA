from bitvector_tools import calcul_gate
GATES = {1: ["id", "not", "<<", ">>"], 2: ["and", "or", "xor"]}

class Node:
    def __init__(self, value = None):
        self.value = value
        self.parent = None
        self.children = []
        self.score = {}
        self.proba = {}
        self.result = None

    def add_child(self, child):
        assert child.parent is None
        self.children.append(child)
        child.parent = self

    def image(self, param = "value", prefixe="", dernier=True):
        node_content = str(self.value)
        if param == "score":
            round_score = {}
            for key in self.score:
                round_score[key] = round(self.score[key], 2)
            node_content = str(round_score)
        if param == "proba":
            round_proba = {}
            for key in self.proba:
                round_proba[key] = round(self.proba[key], 2)
            node_content = str(round_proba)
        if param == "result":
            node_content = str(self.result)
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
    
    def calcul(self, input, nb_bits):
        children_result = [child.calcul(input, nb_bits) for child in self.children]

        self.result = calcul_gate(children_result, self.value, input, nb_bits)
        return self.result
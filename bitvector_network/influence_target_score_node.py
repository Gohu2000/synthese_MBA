from node import Node, GATES
from bitvector_tools import*

class InfluenceTargetScoreNode(Node):
    def __init__(self, value = None):
        super().__init__(value)
        self.influence = None
        self.target = None

    def score_formula(self, instance):
        score = 0
        nb_input = len(instance)
        nb_bits = len(instance[0][0][0])
        for input, y in instance:
            result = self.calcul(input)
            for i in range(nb_bits):
                if result[i] == y[i]:
                    score += 1
        return score / (nb_input*nb_bits)

    def update_tree_score(self, instance):
        nb_bits = len(instance[0][0][0])
        n = len(instance[0][0])
        self.influence = [1 for i in range(nb_bits)]
        nb_input = len(instance)
        self.init_score(n, nb_bits)
        for input, y in instance:
            self.target = y
            result = self.calcul(input)
            self.update_influence_target()
            self.update_score(input, y)
        self.finish_score(nb_input*nb_bits)

    def init_score(self, n, nb_bits):
        arity = len(self.children)
        self.score = {}
        l_gates = list(range(n))
        if arity == 1:
            l_gates = ["id", "not"]
            l_gates += ["<<" + str(i) for i in range(1, nb_bits + 1)]
            l_gates += [">>" + str(i) for i in range(1, nb_bits + 1)]
        if arity == 2:
            l_gates = GATES[arity]

        for gate in l_gates:
            if gate != self.value:
                self.score[gate] = 0

        for child in self.children:
            child.init_score(n, nb_bits)

    def finish_score(self, denominator):
        for gate in self.score:
            self.score[gate] = self.score[gate] / denominator
        for child in self.children:
            child.finish_score(denominator)
    
    def update_score(self, input, y):
        nb_bits = len(input[0])
        
        current_nb_match = 0
        for i in range(nb_bits):
            if self.influence[i] == 1:
                if self.result[i] == y[i]:
                    current_nb_match += 1
            if self.influence[i] == -1:
                if self.result[i] != y[i]:
                    current_nb_match += 1

        for gate in self.score:
            children_result = [child.result for child in self.children]
            result = calcul_gate(children_result, gate, input)
            nb_match = 0
            for i in range(nb_bits):
                if self.influence[i] == 1:
                    if result[i] == y[i]:
                        nb_match += 1
                if self.influence[i] == -1:
                    if result[i] != y[i]:
                        nb_match += 1

            self.score[gate] += nb_match - current_nb_match

        for child in self.children:
            child.update_score(input, y)


    def update_influence_target(self):
        arity = len(self.children)
        nb_bits = len(self.influence)
        antishift = {"<<":">>", ">>":"<<"}

        if arity == 0:
            return
        if arity == 1:
            child = self.children[0]
            if self.value == "id":
                child.influence = self.influence
                child.target = self.target
            if self.value == "not":
                child.influence = self.influence
            if self.value[:2] in ["<<", ">>"]:
                n = int(self.value[2:])
                child.influence = shift(self.influence, n, antishift[self.value[:2]])
            child.update_influence()

        if arity == 2:
            child1 = self.children[0]
            child2 = self.children[1]
            mask1 = [1 for i in range(nb_bits)]
            mask2 = [1 for i in range(nb_bits)]
            assert self.value in GATES[2]
            if self.value == "or":
                mask1 = [1-bit for bit in child2.result]
                mask2 = [1-bit for bit in child1.result]
            if self.value == "and":
                mask1 = child2.result
                mask2 = child1.result
            if self.value == "xor":
                mask1 = [-2*bit + 1 for bit in child2.result]
                mask2 = [-2*bit + 1 for bit in child1.result]

            child1.influence = multiply_list(mask1, self.influence)
            child2.influence = multiply_list(mask2, self.influence)
            child1.update_influence()
            child2.update_influence()
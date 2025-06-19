from node import Node, GATES
from bitvector_tools import hamming_weight, bit_not, calcul_gate, shift

class InfluenceTargetScoreNode(Node):
    def __init__(self, value = None):
        super().__init__(value)
        self.influence = None
        self.target = None
        self.new_influence = None

    def score_formula(self, instance):
        score = 0
        nb_input = len(instance) - 1
        nb_bits = instance[0]
        for input, y in instance[1:]:
            result = self.calcul(input, nb_bits)
            score += hamming_weight(bit_not(y ^ result, nb_bits))
        return score / (nb_input*nb_bits)

    def update_tree_score(self, instance):
        nb_bits = instance[0]
        n = len(instance[1][0])
        self.influence = (1 << nb_bits) - 1
        nb_input = len(instance) - 1
        self.init_score(n, nb_bits)
        for input, y in instance[1:]:
            self.target = y
            self.calcul(input, nb_bits)
            self.update_influence_target(nb_bits)
            self.update_score(input, y, nb_bits)
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
    
    def update_score(self, input, y, nb_bits):
        
        current_nb_match = hamming_weight(bit_not(self.result ^ self.target, nb_bits) & self.influence)

        for gate in self.score:
            children_result = [child.result for child in self.children]
            result = calcul_gate(children_result, gate, input, nb_bits)
            nb_match = hamming_weight(bit_not(result ^ self.target, nb_bits) & self.influence)

            self.score[gate] += nb_match - current_nb_match

        for child in self.children:
            child.update_score(input, y, nb_bits)


    def update_influence_target(self, nb_bits):
        arity = len(self.children)
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
                child.target = bit_not(self.target, nb_bits)
            if self.value[:2] in ["<<", ">>"]:
                m = int(self.value[2:])
                child.influence = shift(self.influence, m, nb_bits, antishift[self.value[:2]])
                child.target = shift(self.target, m, nb_bits, antishift[self.value[:2]])
            child.update_influence_target(nb_bits)

        if arity == 2:
            child1 = self.children[0]
            child2 = self.children[1]
            assert self.value in GATES[2]
            if self.value == "or":
                child1.influence = bit_not(child2.result, nb_bits) & self.influence
                child2.influence = bit_not(child1.result, nb_bits) & self.influence
                child1.target = self.target
                child2.target = self.target
            if self.value == "and":
                child1.influence = child2.result & self.influence
                child2.influence = child1.result & self.influence
                child1.target = self.target
                child2.target = self.target
            if self.value == "xor":
                child1.influence = self.influence
                child2.influence = self.influence
                child1.target = (child2.result & bit_not(self.target, nb_bits)) | (bit_not(child2.result, nb_bits) & self.target)
                child2.target = (child1.result & bit_not(self.target, nb_bits)) | (bit_not(child1.result, nb_bits) & self.target)

            child1.update_influence_target(nb_bits)
            child2.update_influence_target(nb_bits)


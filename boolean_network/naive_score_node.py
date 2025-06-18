from node import Node, GATES

class NaiveScoreNode(Node):
    def __init__(self, value = None):
        super().__init__(value)

    def score_formula(self, instance):
        score = 0
        n = len(instance)
        for input, y in instance:
            result = self.calcul(input)
            if result == y:
                score += 1
        return score / n

    def update_tree_score(self, instance):
        self.update_score(instance, self, self.score_formula(instance))
    
    def update_score(self, instance, root, score):
        arity = len(self.children)
        n = len(instance[0][0])
        current_value = self.value
        self.score = {}
        l_gates = list(range(n))
        if arity != 0:
            l_gates = GATES[arity]

        for i in l_gates:
            if i != current_value:
                self.value = i
                self.score[i] = root.score_formula(instance) - score
        self.value = current_value

        for child in self.children:
            child.update_score(instance, root, score)


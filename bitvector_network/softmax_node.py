from naive_score_node import NaiveScoreNode
from less_naive_score_node import LessNaiveScoreNode
from influence_score_node import InfluenceScoreNode
import random
from math import exp

DEFAULT_TAU = 10
CLASS_SCORE = InfluenceScoreNode

class SoftmaxNode(CLASS_SCORE):
    def __init__(self, value = None, tau = DEFAULT_TAU):
        super().__init__(value)
        self.tau = tau
        self.f = lambda x: exp(self.tau*x)

    def add_child(self, child):
        super().add_child(child)
        child.change_tau(self.tau)

    def change_tau(self, tau):
        self.tau = tau
        self.f = lambda x: exp(self.tau*x)
        for child in self.children:
            child.change_tau(tau)

    def update_tree_proba(self):
        self.update_proba(self, self.get_sum())

    def update_proba(self, root, sum):
        self.proba = {}
        for gate in self.score:
            self.proba[gate] = self.f(self.score[gate]) / sum

        for child in self.children:
            child.update_proba(root, sum)

    def get_sum(self):
        return self.dfs_sum()
    
    def dfs_sum(self):
        node_sum = 0
        for k, v in self.score.items():
            node_sum += self.f(v)

        for child in self.children:
            node_sum += child.dfs_sum()
        return node_sum

    def softmax(self):
        sum = self.get_sum()
        p = random.random()
        x = self.dfs_proba(0, p, sum)
        assert x == -1

    def dfs_proba(self, previous_proba, p, sum):
        node_proba = previous_proba
        for k, v in self.score.items():
            node_proba += self.f(v) / sum
            if p <= node_proba:
                self.value = k
                return -1
            
        for child in self.children:
            node_proba = child.dfs_proba(node_proba, p, sum)
            if node_proba == -1:
                return -1
        
        return node_proba
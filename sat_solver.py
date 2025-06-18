import random
import time
from typing import Tuple

import cvc5
from cvc5 import Kind, TermManager

type Input = list[int]
type Output = int
type Example = Tuple[Input, Output]
type Task = list[Example]


def solve_cvc5(examples: Task, bit_vec_size: int, n_vars: int):
    tm = cvc5.TermManager()
    slv = cvc5.Solver()

    # required options
    slv.setOption("sygus", "true")
    slv.setOption("incremental", "false")
    # slv.setOption("verbosity", "3")

    # set the logic
    slv.setLogic("QF_BV")

    # Declare BitVec type of the desired width
    bit_vec = tm.mkBitVectorSort(bit_vec_size)

    # Declare input variables
    vars = [tm.mkVar(bit_vec, f"x_{i}") for i in range(n_vars)]

    # Declare Grammar non-terminals
    start = tm.mkVar(bit_vec, "Start")

    # Define the rules
    ## Constants
    zero = tm.mkBitVector(bit_vec_size, 0)
    one = tm.mkBitVector(bit_vec_size, 1)

    ## Rules
    # https://cvc5.github.io/docs/cvc5-1.2.1/api/python/base/kind.html
    # maybe useful : https://github.com/cvc5/artifact-fmcad23-sygus/blob/main/grammars/bv.sy
    And = tm.mkTerm(Kind.BITVECTOR_AND, start, start)
    Or = tm.mkTerm(Kind.BITVECTOR_OR, start, start)
    Not = tm.mkTerm(Kind.BITVECTOR_NOT, start)

    # Todo: also allow shifts, etc

    g = slv.mkGrammar(vars, [start])
    g.addRules(start, [zero, one, And, Or, Not] + vars)

    func = slv.synthFun("f", vars, bit_vec, g)
    ## Add constraints from examples.
    # For each example pair
    for inputs, output in examples:
        # Create values from input variables
        input_bitvecs = [tm.mkBitVector(bit_vec_size, x) for x in inputs]
        # Apply f to these values
        f_output = tm.mkTerm(Kind.APPLY_UF, func, *input_bitvecs)
        # Create value for output
        output_bitvec = tm.mkBitVector(bit_vec_size, output)
        # Constraint: value of f must be equal to value of output.
        slv.addSygusConstraint(tm.mkTerm(Kind.EQUAL, f_output, output_bitvec))

    print("starting synth")
    if slv.checkSynth().hasSolution():
        print("Solution found:")
        terms = [func]
        print_synth_solutions(terms, slv.getSynthSolutions(terms))
    else:
        print("No solution found")


def define_fun_to_string(f, params, body):
    sort = f.getSort()
    if sort.isFunction():
        sort = f.getSort().getFunctionCodomainSort()

    result = "(define-fun " + str(f) + " ("
    for i in range(0, len(params)):
        if i > 0:
            result += " "
        result += "(" + str(params[i]) + " " + str(params[i].getSort()) + ")"

    result += ") " + str(sort) + " " + str(body) + ")"
    return result


def print_synth_solutions(terms, sols):
    result = "(\n"
    for i in range(0, len(terms)):
        params = []
        body = sols[i]
        if sols[i].getKind() == Kind.LAMBDA:
            params += sols[i][0]
            body = sols[i][1]
        result += "  " + define_fun_to_string(terms[i], params, body) + "\n"

    result += ")"
    print(result)


class FormulaNode:
    def __init__(self, name, op, *children) -> None:
        self.name = name
        self.op = op
        self.children = children

    def eval(self, inputs, n_bits):
        eval_children = [c.eval(inputs, n_bits) for c in self.children]
        return self.op(n_bits, inputs, *eval_children)

    def to_string(self, prefix="", last=True) -> str:
        r = prefix + ("└── " if last else "├── ") + self.name + "\n"
        for i, c in enumerate(self.children):
            next_last = i == len(self.children) - 1
            r += c.to_string(prefix + ("    " if last else "|   "), next_last)
        # str_children = [c.to_string(prefix + "|   ") for c in self.children]
        # r = prefix + "├── " + self.name + "\n" + "".join(str_children)
        return r


# vars = {i: (lambda inputs, *children: print(len(inputs), i) or inputs[i]) for i in range(10)}
unary = {"not": lambda n_bits, inputs, *children: (~children[0]) & ((1 << n_bits) - 1)}
binary = {
    "or": lambda n_bits, inputs, *children: children[0] | children[1],
    "and": lambda n_bits, inputs, *children: children[0] & children[1],
}


def random_formula(size: int, n_vars: int):
    if size == 1:
        i = random.randrange(n_vars)

        def v(n_bits, inputs, *children):
            return inputs[i]

        n = FormulaNode(f"x_{i}", v)
        return n
    elif size == 2:
        s = random_formula(1, n_vars)
        n = FormulaNode("not", unary["not"], s)
        return n
    else:
        left = random.randint(1, size - 2)
        right = size - 1 - left
        f_left = random_formula(left, n_vars)
        f_right = random_formula(right, n_vars)
        op = random.choice(list(binary.keys()))
        f = binary[op]
        return FormulaNode(op, f, f_left, f_right)


def gen_examples(f: FormulaNode, n_bits: int, n_vars: int, n_examples: int) -> Task:
    ub = 1 << n_bits - 1
    task = []
    for _ in range(n_examples):
        inputs = [random.randint(0, ub) for _ in range(n_vars)]
        output = f.eval(inputs, n_bits)
        task.append((inputs, output))
    return task


if __name__ == "__main__":
    # solve_cvc5(
    #     [
    #         ([0, 1], 0),
    #         ([1, 1], 1),
    #         ([1, 0], 0),
    #         ([0, 0], 0),
    #     ],
    #     bit_vec_size=16,
    #     n_vars=2,
    # )

    N_BITS = 3
    N_VARS = 5
    N_EXAMPLES = 200
    F_SIZE = 15
    f = random_formula(F_SIZE, N_VARS)
    print(f.to_string())
    ex = gen_examples(f, N_BITS, N_VARS, N_EXAMPLES)
    start = time.time()
    solve_cvc5(ex, N_BITS, N_VARS)
    elapsed = time.time() - start
    print(f"Time: {elapsed:.3f}s")

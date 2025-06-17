import copy
from typing import Tuple
import cvc5

# import utils
from cvc5 import TermManager, Kind

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


if __name__ == "__main__":
    solve_cvc5(
        [
            ([0, 1], 0),
            ([1, 1], 1),
            ([1, 0], 0),
            ([0, 0], 0),
        ],
        bit_vec_size=16,
        n_vars=2,
    )

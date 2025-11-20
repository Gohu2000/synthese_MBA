from results import*
from treat_results import*
from algorithms import*
from time import time

def test_evolution(n, size, nb_bits, progressive = True):
    node = create_rd_formula(n, size, nb_bits)
    instance, goal = create_rd_instance(n, size, 2**(n-1), nb_bits)
    score = 0
    if progressive:
        score = progressive_evolution(node, instance, 1000)
    else:
        score = evolution(node, instance, tau =20, N=1000)
    goal.print()
    node.print()
    node.print("score")
    print(score)

def test_seed(n, size, nb_bits):
    node = create_rd_formula(n, size, nb_bits)
    seed = node.get_seed()
    node.print()
    copy = recreate_formula(seed)
    copy.print()

def test_examples():
    generate_examples()
    formula, instance = extract_examples()[0]
    print_example("example/example_0.txt", formula, instance)

def test_example_file(algoname, filename, nb_instances, dir_benchmark):
    result_file = "result_" + algoname + "_of_" + filename
    try:
        i = L_ALGO_STR.index(algoname)
        result_example_file(L_ALGO[i], result_file, filename, nb_instances, dir_benchmark)
    except ValueError:
        if algoname.startswith("algo_softmax"):
            n = len("algo_softmax")
            tau, N, progressive = [int(param) for param in algoname[n:].split("_")]
            result_example_file(algo_softmax(tau, N, progressive), result_file, filename, nb_instances, dir_benchmark)

def test_benchmark(algoname, nb_instances, dir_benchmark):
    result_file = "result_" + algoname + ".txt"
    try:
        i = L_ALGO_STR.index(algoname)
        result_benchmark(L_ALGO[i], result_file, nb_instances, dir_benchmark)
    except ValueError:
        if algoname.startswith("algo_softmax"):
            n = len("algo_softmax")
            tau, N, progressive = [int(param) for param in algoname[n:].split("_")]
            result_benchmark(algo_softmax(tau, N, progressive), result_file, nb_instances, dir_benchmark)


# test_evolution(5, 15, 5, False)
# test_seed(5, 15, 5)
# test_examples()

# generate_benchmark(5, 15, g, 5)
def benchmark1():
    l_n = [5]
    l_size = [10]
    l_nb_bits=[32]
    l_nb_inputs=[128]
    nb_instances = 100
    dir_benchmark = "benchmark2"
    filename = "5_32_10.txt"

    generate_benchmark(l_n, l_size, l_nb_bits, l_nb_inputs, nb_instances, dir_benchmark)

    algonames_sup = ["algo_die_retry"]
    algonames = L_ALGO_STR + algonames_sup
    
    for algoname in algonames_sup:
        t0 = time()
        test_example_file(algoname, filename, nb_instances, dir_benchmark)
        result_file = "result_" + algoname + "_of_" + filename
        exploit_results(basic_exploit, l_nb_inputs, result_file, "result_benchmark2.txt")
        print(time() - t0)



benchmark1()
# algonames = ["algo_softmax20_1000_1", "algo_softmax20_1000_0", "algo_softmax50_1000_0", "algo_softmax50_1000_1"]
"""
algonames = ["algo_die_retry"]
nb_instances = 100
dir_benchmark = "benchmark1"
filename = "5_5_10.txt"
t0 = time()
    
for algoname in algonames:
    t0 = time()
    test_example_file(algoname, filename, nb_instances, dir_benchmark)
    result_file = "result_" + algoname + "_of_" + filename
    exploit_results(basic_exploit, [32], result_file, "result_benchmark1_.txt")

    print(time() - t0)"""
# test_benchmark("algo_softmax_progressif")
# test_example_file("algo_softmax_progressif")

#generate_examples(filename = "test.txt", params = [(5, 15, 3, 5)])

#formula, instance = extract_examples("benchmark1/5_5_10.txt")[0]
#print_example("example_test2.txt", formula, instance, tau = default_tau)

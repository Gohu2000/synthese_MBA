from init_functions import*
from time import time

def evolution(f, instance, tau = 5, N = 10):
    score = 0
    f.change_tau(tau)
    for i in range(N):
        score = f.score_formula(instance)
        # print(score)
        f.update_tree_score(instance)
        if score > 0.99:
            return score
        f.softmax()

    return score

def progressive_evolution(f, instance, N, tau_max=10):
    score = 0
    tau = 1
    # (tau_max - tau)/step + 1 = N / 10
    step = 10*(tau_max - tau)/(N-10)
    for i in range(N//10):
        score = evolution(f, instance, tau)
        if score > 0.99:
            return score
        tau += step

    return score

def algo_random(instance, n, size, nb_bits):
    new_formula = create_rd_formula(n, size, nb_bits)
    return new_formula.score_formula(instance)

def algo_random_brute_force(instance, n, size, nb_bits):
    new_formula = create_rd_formula(n, size, nb_bits)
    return evolution(new_formula, instance, tau = 0, N=100)

def algo_softmax_tau_eleve(instance, n, size, nb_bits):
    new_formula = create_rd_formula(n, size, nb_bits)
    return evolution(new_formula, instance, tau = 20, N=100)

def algo_softmax_tau_moyen(instance, n, size, nb_bits):
    new_formula = create_rd_formula(n, size, nb_bits)
    return evolution(new_formula, instance, tau = 8, N=100)

def algo_softmax_progressif(instance, n, size, nb_bits):
    new_formula = create_rd_formula(n, size, nb_bits)
    return progressive_evolution(new_formula, instance, N=100, tau_max=10)

def algo_die_retry(instance, n, size, nb_bits):
    score = 0
    for i in range(100):
        new_formula = create_rd_formula(n, size, nb_bits)
        score = max(evolution(new_formula, instance, tau = 30, N=100), score)
        if score > 0.99:
            return score
    return score

def raw_algo_softmax(tau, N, instance, n, size, nb_bits, progressive):
    new_formula = create_rd_formula(n, size, nb_bits)
    if progressive:
        return progressive_evolution(new_formula, instance, N, tau)
    return evolution(new_formula, instance, tau, N)

def algo_softmax(tau, N, progressive = False):
    # tau ou tau_max selon progressive
    return lambda instance, n, size, nb_bits: raw_algo_softmax(tau, N, instance, n, size, nb_bits, progressive)

L_ALGO = [algo_random, algo_random_brute_force, algo_softmax_tau_eleve, algo_softmax_tau_moyen, algo_softmax_progressif, algo_die_retry]
L_ALGO_STR = ["algo_random", "algo_random_brute_force", "algo_softmax_tau_eleve", "algo_softmax_tau_moyen", "algo_softmax_progressif", "algo_die_retry"]
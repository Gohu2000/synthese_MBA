from examples import*

def evolution(f, instance, tau = 5, N = 10):
    score = 0
    f.change_tau(tau)
    for i in range(N):
        score = f.score_formula(instance)
        print(score)
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


def test_evolution(n, size, progressive = True):
    node = create_rd_formula(n, size)
    instance, goal = create_rd_instance(n, size, 2**(n-1))
    score = 0
    if progressive:
        score = progressive_evolution(node, instance, 1000)
    else:
        score = evolution(node, instance, tau =20, N=1000)
    goal.print()
    node.print()
    node.print("score")
    print(score)

def test_seed(n, size):
    node = create_rd_formula(n, size)
    seed = node.get_seed()
    node.print()
    copy = recreate_formula(seed)
    copy.print()

def test_examples():
    generate_examples()
    formula, instance = extract_examples()[0]
    print_example("example_0.txt", formula, instance)

# test_evolution(5, 15, False)
# test_evolution(5, 15, False)
# test_seed(5, 15)
test_examples()
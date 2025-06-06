def int_to_binary(n, size):
    assert 0 <= n < 2**size
    if n == 0:
        return [0 for i in range(size)]
    q, r = divmod(n, 2)
    return [r] + int_to_binary(q, size-1)

def binary_to_int(bitvector):
    n = 0
    x = 1
    for b in bitvector:
        n += x*b
        x *= 2
    return n

def instance_to_seed(instance):
    seed = ""
    for input, y in instance:
        seed += str(binary_to_int(input)) + ":" + str(y) + " "
    return seed[:-1]

def seed_to_instance(n, seed):
    l_seed = seed.split()
    instance = []
    for subseed in l_seed:
        l = subseed.split(":")
        input = int_to_binary(int(l[0]), n)
        y = int(l[1])
        instance.append((input, y))
    return instance

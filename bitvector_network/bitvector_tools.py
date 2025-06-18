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
        for bitvector in input:
            seed += str(binary_to_int(bitvector)) + ":"
        seed += str(binary_to_int(y)) + " "
    return seed[:-1]

def seed_to_instance(nb_bits, seed):
    l_seed = seed.split()
    instance = []
    for subseed in l_seed:
        l_subseed = subseed.split(":")
        input = []
        for n in l_subseed[:-1]:
            input.append(int_to_binary(int(n), nb_bits))
        y = int_to_binary(int(l_subseed[-1]), nb_bits)
        instance.append((input, y))
    return instance

"""Opérations sur les bitvectors sous forme de liste"""
def shift(bitvector, n, shift):
    assert shift in ["<<", ">>"]
    m = len(bitvector)
    if n == 0:
        return bitvector
    if n >= m:
        return [0 for i in range(m)]
    if shift == "<<":
        return bitvector[n:] + [0 for i in range(n)]
    if shift == ">>":
        return [0 for i in range(n)] + bitvector[:m-n]
    
def and_bitv(a, b):
    n = len(a)
    assert n == len(b)
    c = []
    for i in range(n):
        c.append(a[i] and b[i])
    return c

def or_bitv(a, b):
    n = len(a)
    assert n == len(b)
    c = []
    for i in range(n):
        c.append(a[i] or b[i])
    return c

def xor_bitv(a, b):
    n = len(a)
    assert n == len(b)
    c = []
    for i in range(n):
        c.append(a[i] ^ b[i])
    return c

def not_bitv(a):
    c = []
    for b in a:
        c.append(1-b)
    return c

def multiply_list(l_1, l_2):
    n = len(l_1)
    assert n == len(l_2)
    l = []
    for i in range(n):
        l.append(l_1[i]*l_2[i])
    return l

def calcul_gate(children_result, value, input):
    arity = len(children_result)
    if arity == 0:
        return input[value]
    elif arity == 1:
        result = children_result[0]
        if value == "not":
            return not_bitv(result)
        if value == "id":
            return result
        if value[:2] in ["<<", ">>"]:
            n = int(value[2:])
            return shift(result, n, value[:2])
    elif arity == 2:
        result1 = children_result[0]
        result2 = children_result[1]
        if value == "or":
            return or_bitv(result1, result2)
        if value == "and":
            return and_bitv(result1, result2)
        if value == "xor":
            return xor_bitv(result1, result2)


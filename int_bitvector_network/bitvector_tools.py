def instance_to_seed(instance):
    seed = ""
    for input, y in instance[1:]:
        for integer in input:
            seed += str(integer) + ":"
        seed += str(y) + " "
    return seed[:-1]

def seed_to_instance(nb_bits, seed):
    l_seed = seed.split()
    instance = [nb_bits]
    for subseed in l_seed:
        l_subseed = subseed.split(":")
        input = []
        for n in l_subseed[:-1]:
            input.append(int(n))
        y = int(l_subseed[-1])
        instance.append((input, y))
    return instance

def bit_not(n, nb_bits):
    return (1 << nb_bits) - 1 - n

def shift(n, m, nb_bits, shift):
    if shift == "<<":
        return (n << m) & ((1 << nb_bits) - 1)
    elif shift == ">>":
        return n >> m
    else:
        raise ValueError
    
def hamming_weight(n):
    result = 0
    while n:
        result += 1
        n &= n-1
    return result

def calcul_gate(children_result, value, input, nb_bits):
    arity = len(children_result)
    if arity == 0:
        return input[value]
    elif arity == 1:
        result = children_result[0]
        if value == "not":
            return bit_not(result, nb_bits)
        if value == "id":
            return result
        if value[:2] in ["<<", ">>"]:
            m = int(value[2:])
            return shift(result, m, nb_bits, value[:2])
    elif arity == 2:
        result1 = children_result[0]
        result2 = children_result[1]
        if value == "or":
            return result1 | result2
        if value == "and":
            return result1 & result2
        if value == "xor":
            return result1 ^ result2

def new_and(x, y, z, nb_bits):
    y_s = y
    star = -1
    for i in range(nb_bits):
        x_i = x & (1 << i)
        y_i = y & (1 << i)
        z_i = z & (1 << i)
        if y_i:
            star = i
            y_star = y_i
        if not z_i:
            if star == -1 and x_i:
                return -1, -1
            else:
                y_s -= y_star
                if x_i:
                    x += 1 << star
                star = -1
        if not x_i:
            star = -1
        print(x, star)
    return x & ((1<<nb_bits)-1), y_s | bit_not(z, nb_bits)

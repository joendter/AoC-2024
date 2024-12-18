start = {125:1, 17:1}
real = {4:1, 4841539:1, 66:1, 5279:1, 49207:1, 134:1, 609568:1, 0:1}


def add(dictionary, index, amount):
    if index in dictionary.keys():
        dictionary[index] = dictionary[index] + amount
    else:
        dictionary[index] =  amount
def next(current):
    res = {}
    for i in current.keys():
        if i == 0:
            add(res, 1, current[i])
            continue
        stringified = str(i)
        length = len(stringified)
        if length % 2 == 0:
            modulo = int(10**(length/2))
            add(res, i // modulo, current[i])
            add(res, i % modulo, current[i])
            continue
        add(res, i*2024, current[i])
    return res

print(next(start))

def iterate(base, iterations):
    current = base
    for i in range(iterations):
        current = next(current)
    return current

def summarise(result):
    sum_a= 0
    for i in result.keys():
        sum_a += result[i]
    return sum_a

print(summarise(iterate(real,75)))
print(summarise(iterate(real,1000)))

#print(len(iterate(start, 25)))
#print(iterate(start, 6))
cmp = {}
for i in [2097446912 ,14168 ,4048 ,2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2]:
    add(cmp, i, 1)
#print(cmp)


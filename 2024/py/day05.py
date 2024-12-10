import functools
import os


with open(os.path.join(os.path.abspath(os.path.dirname(__file__)), "../input/day05.txt"), "r") as f:
    rules = {}
    while (line := f.readline().strip()) != '':
        l = line.split('|')
        r = rules.setdefault(l[0], [])
        r.append(l[1])

    sum1 = 0
    sum2 = 0
    while (line := f.readline().strip()) != '':
        updates = line.split(',')

        in_order = True
        for i, update in enumerate(updates[::-1]):
            if any(x in rules[update] for x in updates[:-i]):
                in_order = False
                break

        if in_order:
            sum1 += int(updates[len(updates)//2])
        else:
            updates.sort(key=functools.cmp_to_key(lambda x,y: 1 if x in rules[y] else -1))
            sum2 += int(updates[len(updates)//2])

    print(sum1)
    print(sum2)

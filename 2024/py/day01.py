import os


with open(os.path.join(os.path.abspath(os.path.dirname(__file__)), "../input/day01.txt"), "r") as f:
    l = []
    r = []
    for line in f.readlines():
        split_line = line.split()
        l.append(int(split_line[0]))
        r.append(int(split_line[1]))

l.sort()
r.sort()

# p1

sum = sum(abs(a - b) for (a, b) in zip(l, r))
print(sum)

# p2

s = {}
for id in l:
    if id in s:
        continue

    count = 0
    for rep in r:
        if rep == id:
            count += 1
    s[id] = count

sum = 0
for (id, count) in s.items():
    sum += (id * count)

print(sum)

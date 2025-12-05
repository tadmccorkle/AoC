import os
import time


# with open(os.path.join(os.path.abspath(os.path.dirname(__file__)), "../input/sample.txt"), "r") as f:
with open(os.path.join(os.path.abspath(os.path.dirname(__file__)), "../input/day01.txt"), "r") as f:
    rots = f.readlines()

# p1

start = time.perf_counter()
zeroes = 0
point = 50
for rot in rots:
    diff = 100 - int(rot[1:]) if rot[0] == 'L' else int(rot[1:])
    point = ((point + diff) % 100)
    if point == 0:
        zeroes += 1
end = time.perf_counter()

print(f'P1: {zeroes} (time: {end - start:.6f} s)')

# p2

start = time.perf_counter()
zeroes = 0
point = 50
for rot in rots:
    diff = int(rot[1:])
    if rot[0] == 'L':
        p = point - diff
        if p == 0 and point > 0:
            zeroes += 1
        elif p < 0:
            if point > 0:
                zeroes += 1
            zeroes += abs(p) // 100
            p = (p + 100) % 100
        point = p
    else:
        point += diff
        zeroes += point // 100
        point %= 100
end = time.perf_counter()

print(f'P2: {zeroes} (time: {end - start:.6f} s)')

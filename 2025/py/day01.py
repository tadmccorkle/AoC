import os


# with open(os.path.join(os.path.abspath(os.path.dirname(__file__)), "../input/sample.txt"), "r") as f:
with open(os.path.join(os.path.abspath(os.path.dirname(__file__)), "../input/day01.txt"), "r") as f:
    rots = f.readlines()

# p1

zeroes = 0
point = 50
for rot in rots:
    diff = 100 - int(rot[1:]) if rot[0] == 'L' else int(rot[1:])
    point = ((point + diff) % 100)
    if point == 0: 
        zeroes += 1
print(zeroes)

# p2

print("P2: TODO")

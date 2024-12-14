import os


with open(os.path.join(os.path.abspath(os.path.dirname(__file__)), "../input/day10.txt"), "r") as f:
    grid = [list(line.strip()) for line in f]

MIN_X = MIN_Y = 0
MAX_X = len(grid[0])
MAX_Y = len(grid)

def traverse(x, y, s):
    curr = grid[y][x]

    if curr == '9':
        s.add((x, y))
        return 1

    score = 0
    next = int(grid[y][x]) + 1
    if x > 0 and next == int(grid[y][x - 1]):
        score += traverse(x - 1, y, s)
    if y > 0 and next == int(grid[y - 1][x]):
        score += traverse(x, y - 1, s)
    if x < MAX_X - 1 and next == int(grid[y][x + 1]):
        score += traverse(x + 1, y, s)
    if y < MAX_Y - 1 and next == int(grid[y + 1][x]):
        score += traverse(x, y + 1, s)
    return score

sum_1 = sum_2 = 0
for y, r in enumerate(grid):
    for x, c in enumerate(r):
        if c != '0':
            continue

        score = set()
        sum_2 += traverse(x, y, score)
        sum_1 += len(score)

print(sum_1, sum_2)

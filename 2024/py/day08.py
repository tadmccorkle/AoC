import os


with open(os.path.join(os.path.abspath(os.path.dirname(__file__)), "../input/day08.txt"), "r") as f:
    grid = [list(line.strip()) for line in f]

MIN_X = MIN_Y = 0
MAX_X = len(grid[0])
MAX_Y = len(grid)

antinodes = set()
antinodes_2 = set()

for y, row in enumerate(grid):
    for x, f in enumerate(row):
        if not ('a' <= f <= 'z' or 'A' <= f <= 'Z' or '0' <= f <= '9'):
            continue

        for other_y, other_row in enumerate(grid):
            for other_x, other_f in enumerate(other_row):
                if f != other_f or (x == other_x and y == other_y):
                    continue

                dx, dy = x - other_x, y - other_y
                ax, ay = x + dx, y + dy
                if MIN_X <= ax < MAX_X and MIN_Y <= ay < MAX_Y:
                    antinodes.add((ax, ay))

                antinodes_2.add((x, y))
                while MIN_X <= ax < MAX_X and MIN_Y <= ay < MAX_Y:
                    antinodes_2.add((ax, ay))
                    ax, ay = ax + dx, ay + dy

print(len(antinodes))
print(len(antinodes_2))

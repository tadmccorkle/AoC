import os


with open(os.path.join(os.path.abspath(os.path.dirname(__file__)), "../input/day06.txt"), "r") as f:
    start = 0, 0
    grid = []
    for i, line in enumerate(f):
        if line == "": break
        j = line.find("^")
        if j != -1:
            start = j, i
        grid.append(list(line.strip()))


def is_off(grid, x, y):
    return x < 0 or x == len(grid[0]) or y < 0 or y == len(grid)

def get_visits(grid):
    visits = set()
    loc_x, loc_y = start
    dx = 0
    dy = -1

    while True:
        visits.add((loc_x, loc_y))

        x, y = loc_x + dx, loc_y + dy
        while not (off_grid := is_off(grid, x, y)) and grid[y][x] == "#":
            dx, dy = dy * -1, dx
            x, y = loc_x + dx, loc_y + dy
        if off_grid:
            return visits

        loc_x, loc_y = x, y

def is_loop(grid):
    visits = set()
    loc_x, loc_y = start
    dx = 0
    dy = -1

    while True:
        visit = (loc_x, loc_y, dx, dy)
        if visit in visits:
            return True
        visits.add(visit)

        x, y = loc_x + dx, loc_y + dy
        while not (off_grid := is_off(grid, x, y)) and grid[y][x] == "#":
            dx, dy = dy * -1, dx
            x, y = loc_x + dx, loc_y + dy
        if off_grid:
            return False

        loc_x, loc_y = x, y


visits = get_visits(grid)
print(len(visits))

sum = 0
visits.remove(start)
for x, y in visits:
    grid[y][x] = "#"
    if is_loop(grid):
        sum += 1
    grid[y][x] = "."
print(sum)

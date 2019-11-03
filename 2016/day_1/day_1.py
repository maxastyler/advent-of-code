with open("./input", "r") as f:
    instructions = [(1j if x[0] == 'L' else -1j, int(x[1:]))
                    for x in [x.strip() for x in f.read().split(",")]]

positions = [0]
rotation = 1j

for (r, d) in instructions:
    rotation *= r
    for i in range(d):
        positions.append(positions[-1] + rotation)

retreads = [p for (i, p) in enumerate(positions) if p in positions[:i]]

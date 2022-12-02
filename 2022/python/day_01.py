with open("../inputs/day_01") as input:
    calories = [sum(int(n) for n in l.splitlines()) for l in input.read().split("\n\n")]
    print(f"Part 1: {max(calories)}")
    print(f"Part 2: {sum(sorted(calories)[-3:])}")

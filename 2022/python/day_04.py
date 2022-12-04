import re


def convert_line(line: str) -> tuple[tuple[int, int], tuple[int, int]]:
    a, b, c, d = re.match("(\d+)-(\d+),(\d+)-(\d+)", line).groups()
    return ((int(a), int(b)), (int(c), int(d)))


def contains(pair: tuple[tuple[int, int], tuple[int, int]]) -> bool:
    ((a, b), (c, d)) = pair
    return (a <= c and b >= d) or (a >= c and b <= d)


def overlaps(pair: tuple[tuple[int, int], tuple[int, int]]) -> bool:
    ((a, b), (c, d)) = pair
    return (a <= c <= b) or (a <= d <= b) or (c <= a <= d) or (c <= b <= d)


with open("../inputs/day_04") as f:
    pairs = [convert_line(l) for l in f.read().splitlines()]
    print(f"Part 1: {sum(map(contains, pairs))}")
    print(f"Part 2: {sum(map(overlaps, pairs))}")

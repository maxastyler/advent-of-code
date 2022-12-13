from functools import cmp_to_key
from itertools import zip_longest
from typing import Literal


def parse_input(input: str) -> list[tuple[list, list]]:
    return [tuple(eval(x) for x in pair.split("\n")) for pair in input.split("\n\n")]


def check_order(left: list, right: list) -> Literal[-1, 0, 1]:
    listify = lambda x: x if isinstance(x, list) else [x]
    for (l, r) in zip_longest(left, right, fillvalue=None):
        if l is None:
            return -1
        elif r is None:
            return 1
        elif isinstance(l, int) and isinstance(r, int):
            if l == r:
                continue
            elif l < r:
                return -1
            else:
                return 1
        elif (v := check_order(listify(l), listify(r))) != 0:
            return v
    return 0


def part_1(pairs: list[tuple[list, list]]) -> int:
    return sum(i + 1 for (i, (l, r)) in enumerate(pairs) if check_order(l, r) == -1)


def part_2(inputs: list[list]) -> int:
    sorted_list = sorted(inputs + [[[2]], [[6]]], key=cmp_to_key(check_order))
    i1, i2 = sorted_list.index([[2]]), sorted_list.index([[6]])
    return (i1 + 1) * (i2 + 1)


if __name__ == "__main__":
    with open("../inputs/day_13") as f:
        input = f.read()
        pairs = parse_input(input)
        print(f"Part 1: {part_1(pairs)}")
        print(f"Part 2: {part_2([i for pair in pairs for i in pair])}")

import re
from itertools import accumulate

from pyrsistent import pvector
from pyrsistent.typing import PVector


def sign(x: int) -> int:
    if x > 0:
        return 1
    elif x == 0:
        return 0
    else:
        return -1


def move_head(head_pos: tuple[int, int], instruction: str) -> tuple[int, int]:
    match instruction:
        case "U":
            return (head_pos[0], head_pos[1] + 1)
        case "D":
            return (head_pos[0], head_pos[1] - 1)
        case "L":
            return (head_pos[0] - 1, head_pos[1])
        case "R":
            return (head_pos[0] + 1, head_pos[1])
        case _:
            raise ValueError()


def move_tail(head_pos: tuple[int, int], tail_pos: tuple[int, int]) -> tuple[int, int]:
    (hx, hy), (tx, ty) = head_pos, tail_pos
    distance = (hx - tx) ** 2 + (hy - ty) ** 2
    if distance >= 4:
        if hx == tx:
            return (tx, ty + sign(hy - ty))
        elif hy == ty:
            return (tx + sign(hx - tx), ty)
        else:
            return (tx + sign(hx - tx), ty + sign(hy - ty))
    else:
        return tail_pos


def reducer(
    acc: tuple[tuple[int, int], PVector[tuple[int, int]]], inst: str
) -> tuple[tuple[int, int], PVector[tuple[int, int]]]:
    new_head = move_head(acc[0], inst)
    tail = acc[1]
    new_tail = pvector(accumulate(tail, move_tail, initial=new_head))[1:]
    return (new_head, new_tail)


if __name__ == "__main__":
    with open("../inputs/day_09") as f:
        instructions = [
            a
            for (a, b) in re.findall("([UDLR]) (\d+)", f.read())
            for i in range(int(b))
        ]
        positions = accumulate(instructions, reducer, initial=((0, 0), pvector(((0, 0),))))
        print(f"Part 1: {len(set(p for (_, p) in positions))}")
        p2pos = accumulate(instructions, reducer, initial=((0, 0), pvector([(0, 0)]*9)))
        print(f"Part 2: {len(set(p[-1] for (_, p) in p2pos))}")

import itertools
import operator
from functools import reduce
from typing import Any

from attr import frozen, field
from pyrsistent import pvector, pmap
from pyrsistent.typing import PVector, PMap


@frozen
class Indexer:
    shape: tuple[int, int] = field()

    def __call__(self, array: PVector[int], x: int, y: int) -> int:
        return array[y * shape[0] + x]


def transpose(vec: PVector[PVector[Any]]) -> PVector[PVector[Any]]:
    return pvector(pvector(x) for x in zip(*vec))


def visible(
    trees: PVector[int], shape: tuple[int, int]
) -> PVector[tuple[bool, bool, bool, bool]]:
    i = Indexer(shape)

    def reducer(
        acc: tuple[int, PVector[bool]], position: tuple[int, int]
    ) -> tuple[int, PVector[bool]]:
        maximum, current = acc
        v = i(trees, *position)
        return (v if v > acc[0] else acc[0], acc[1].append(v > maximum))

    left, right, up, down = pvector(), pvector(), pvector(), pvector()
    for row in range(shape[1]):
        left_to_right = [(i, row) for i in range(shape[0])]
        left = left.extend(reduce(reducer, left_to_right, (-1, pvector()))[1])
        right = right.extend(
            reduce(reducer, left_to_right[::-1], (-1, pvector()))[1][::-1]
        )
    for col in range(shape[0]):
        up_to_down = [(col, i) for i in range(shape[1])]
        up = up.append(reduce(reducer, up_to_down, (-1, pvector()))[1])
        down = down.append(reduce(reducer, up_to_down[::-1], (-1, pvector()))[1][::-1])
    up = pvector(j for i in transpose(up) for j in i)
    down = pvector(j for i in transpose(down) for j in i)
    return pvector(zip(left, right, up, down))


def score(trees: PMap[tuple[int, int], int], pos: tuple[int, int]) -> int:
    score = 1
    for direction in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
        start_height = trees[pos]
        distance = 0
        cur_pos = pos
        while True:
            cur_pos = (cur_pos[0] + direction[0], cur_pos[1] + direction[1])
            try:
                if trees[cur_pos] >= start_height:
                    distance += 1
                    break
                else:
                    distance += 1
            except KeyError:
                break
        score *= distance
    return score


def create_map(input: str) -> PMap[tuple[int, int], int]:
    return pmap(
        {
            (i, j): int(tree)
            for (i, row) in enumerate(input.split())
            for (j, tree) in enumerate(row)
        }
    )


if __name__ == "__main__":
    with open("../inputs/day_08") as f:
        input = f.read()
        rows = input.split()
        shape = (len(rows[0]), len(rows))
        trees = pvector(int(i) for row in rows for i in row)
        tree_visibility = visible(trees, shape)
        print(f"Part 1: {sum(reduce(operator.or_, i, False) for i in tree_visibility)}")
        tree_map = create_map(input)
        print(f"Part 2: {max(score(tree_map, i) for i in tree_map.keys())}")

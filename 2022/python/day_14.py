import itertools as i
import re
from typing import Iterator

from pyrsistent import pset
from pyrsistent.typing import PSet


def parse_input(input: str) -> PSet[tuple[int, int]]:
    coord_re = re.compile("(-?\d+),(-?\d+)")

    def parse_line(line: str) -> list[tuple[int, int]]:
        return [(int(x), int(y)) for (x, y) in coord_re.findall(line)]

    def produce_range(a: int, b: int) -> Iterator[int]:
        return [a] if a == b else i.chain(range(a, b, (b - a) // abs(b - a)), [b])

    def get_points(a: tuple[int, int], b: tuple[int, int]) -> list[tuple[int, int]]:
        return list(i.product(produce_range(a[0], b[0]), produce_range(a[1], b[1])))

    def fill_path(points: list[tuple[int, int]]) -> list[tuple[int, int]]:
        return [p for (a, b) in zip(points, points[1:]) for p in get_points(a, b)]

    return pset(p for l in input.split("\n") for p in fill_path(parse_line(l)))


fall_vectors = [(0, 1), (-1, 1), (1, 1)]


def fall_step(
    position: tuple[int, int],
    blocking: PSet[tuple[int, int]],
    bottom: int | None,
) -> tuple[int, int] | None:
    (x, y) = position
    if bottom is not None and y + 1 == bottom:
        return None
    for (dx, dy) in fall_vectors:
        if (new_pos := (x + dx, y + dy)) not in blocking:
            return new_pos


def sandfall(
    position: tuple[int, int],
    blocking: PSet[tuple[int, int]],
    bottom: int | None = None,
) -> tuple[int, int] | None:
    yield position
    while (new_pos := fall_step(position, blocking, bottom)) is not None:
        position = new_pos
        yield position
    return


def part_1(blocks: PSet[tuple[int, int]]) -> int:
    max_block = max(blocks, key=lambda x: x[1])[1]
    resting = pset()
    while True:
        stable = list(
            i.takewhile(
                lambda x: x[1] <= max_block, sandfall((500, 0), blocks.union(resting))
            )
        )[-1]
        if stable[1] >= max_block:
            return len(resting)
        else:
            resting = resting.add(stable)


def part_2(blocks: PSet[tuple[int, int]]) -> int:
    y_line = 2 + max(blocks, key=lambda p: p[1])[1]
    resting = 0
    while True:
        stable = list(sandfall((500, 0), blocks, bottom=y_line))[-1]
        if stable == (500, 0):
            return resting + 1
        else:
            resting += 1
            blocks = blocks.add(stable)


if __name__ == "__main__":
    with open("../inputs/day_14") as f:
        input = f.read()
        rocks = parse_input(input)
        settled = pset()
        print(part_1(rocks))
        print(part_2(rocks))

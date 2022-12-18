import re
from typing import Iterator

from pyrsistent import pdeque, pset
from pyrsistent.typing import PSet


def neighbours(cube: tuple[int, int, int]) -> Iterator[tuple[int, int, int]]:
    (x, y, z) = cube
    yield (x + 1, y, z)
    yield (x - 1, y, z)
    yield (x, y + 1, z)
    yield (x, y - 1, z)
    yield (x, y, z + 1)
    yield (x, y, z - 1)


def count_exposed(cubes: PSet[tuple[int, int, int]]) -> int:
    total = 0
    for cube in cubes:
        for neighbour in neighbours(cube):
            if neighbour not in cubes:
                total += 1
    return total


def over_bounds(
    cubes: PSet[tuple[int, int, int]]
) -> tuple[tuple[int, int, int], tuple[int, int, int]]:
    extract = lambda f, i: f(c[i] for c in cubes)
    return tuple(min(c[i] for c in cubes) - 1 for i in range(3)), tuple(
        max(c[i] for c in cubes) + 1 for i in range(3)
    )


def surface_search(
    cubes: PSet[tuple[int, int, int]]
) -> tuple[PSet[tuple[int, int, int]], int]:
    (min_x, min_y, min_z), (max_x, max_y, max_z) = over_bounds(cubes)

    def in_bounds(point: tuple[int, int, int]) -> bool:
        (x, y, z) = point
        return (
            min_x <= x <= max_x and min_y <= y <= max_y and min_z <= z <= max_z
        )

    open_set = pdeque([(min_x, min_y, min_z)])
    searched = pset(pre_size=len(cubes))
    outside = pset(pre_size=len(cubes))
    surface_area = 0

    while len(open_set) > 0:
        current, open_set = open_set.left, open_set.popleft()
        for neighbour in neighbours(current):
            if in_bounds(neighbour) and neighbour not in searched:
                if neighbour in cubes:
                    surface_area += 1
                    outside = outside.add(neighbour)
                elif neighbour not in open_set:
                    open_set = open_set.append(neighbour)
        searched = searched.add(current)
    return outside, surface_area


with open("../inputs/day_18") as f:
    cubes = pset(
        (int(x), int(y), int(z))
        for (x, y, z) in re.findall("(\d+),(\d+),(\d+)", f.read())
    )

    print(f"Part 1: {count_exposed(cubes)}")
    print(f"Part 2: {surface_search(cubes)[1]}")

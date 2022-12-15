import re
from functools import reduce
from typing import Set

from pyrsistent import pvector
from pyrsistent.typing import PVector

Point = tuple[int, int]


def add_range(
    ranges: PVector[tuple[int, int]], x: tuple[int, int]
) -> PVector[tuple[int, int]]:
    found_indices = set()
    (x1, x2) = x
    for i, (r1, r2) in enumerate(ranges):
        if r1 <= x1 <= r2 or r1 <= x2 <= r2 or (x1 <= r1 and x2 >= r2):
            found_indices.add(i)
    points = [p for i in found_indices for p in ranges[i]] + [x1, x2]
    new_range = (min(points), max(points))
    for i in sorted(found_indices, reverse=True):
        ranges = ranges.delete(i)
    return ranges.append(new_range)


def add_range_p2(ranges: list[tuple[int, int]], new_range: tuple[int, int]):
    (x1, x2) = new_range
    min_p, max_p = x1, x2
    found_indices = []
    for i, (r1, r2) in enumerate(ranges):
        if r1 <= x1 <= r2 or r1 <= x2 <= r2:
            found_indices.insert(0, i)
            min_p = min(min_p, r1)
            max_p = max(max_p, r2)
        elif x1 <= r1 and x2 >= r2:
            found_indices.insert(0, i)
    i = 0
    min_p = max(min_p, 0)
    max_p = min(max_p, 4000000)
    if len(found_indices) > 0:
        for i in found_indices:
            ranges.pop(i)
    ranges.insert(i, (min_p, max_p))


def parse_input(input: str) -> list[tuple[Point, Point]]:
    return [
        ((int(sx), int(sy)), (int(bx), int(by)))
        for (sx, sy, bx, by) in re.findall(
            "Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
            input,
        )
    ]


def intersection(radius: int, sensor_position: int, line_position: int) -> int | None:
    distance = abs(line_position - sensor_position)
    if distance <= radius:
        return radius - distance
    else:
        return None


def radius(sensor: Point, beacon: Point) -> int:
    return abs(sensor[0] - beacon[0]) + abs(sensor[1] - beacon[1])


def intersection_ends(
    point: Point, radius: int, line: int, horizontal: bool
) -> tuple[Point, Point] | None:
    (x, y) = point
    if (extent := intersection(radius, y if horizontal else x, line)) is not None:
        if horizontal:
            return (x - extent, line), (x + extent, line)
        else:
            return (line, y - extent), (line, y + extent)


def part_1(signals: list[tuple[Point, Point]]) -> int:
    p = [
        (v[0][0], v[1][0])
        for (s, _, r) in signals
        if (v := intersection_ends(s, r, 2000000, True)) is not None
    ]
    ranges = reduce(add_range, p, pvector())
    return sum(b - a for (a, b) in ranges)


def part_2(signals: list[tuple[Point, Point, int]]) -> int:
    for i in range(4000001):
        p = [
            (v[0][0], v[1][0])
            for (s, _, r) in signals
            if (v := intersection_ends(s, r, i, True)) is not None
        ]
        ranges = []
        for x in p:
            add_range_p2(ranges, x)
        if len(ranges) > 1:
            return i, ranges
    # return sum(b-a for (a, b) in ranges)


if __name__ == "__main__":
    with open("../inputs/day_15") as f:
        signals = [(s, b, radius(s, b)) for (s, b) in parse_input(f.read())]
        print(part_1(signals))
        print(part_2(signals))

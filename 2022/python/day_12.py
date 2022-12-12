import heapq
from typing import Callable

from pyrsistent import pmap, pvector
from pyrsistent.typing import PMap, PVector


def parse_input(
    input: str,
) -> tuple[tuple[int, int], tuple[int, int], PMap[tuple[int, int], int]]:
    output = pmap(pre_size=len(input))
    start, end = (-1, -1), (-1, -1)
    for (r, row) in enumerate(input.split()):
        for (c, col) in enumerate(row):
            match col:
                case "S":
                    start = (r, c)
                    output = output.set((r, c), 0)
                case "E":
                    end = (r, c)
                    output = output.set((r, c), ord("z") - ord("a"))
                case _:
                    output = output.set((r, c), ord(col) - ord("a"))
    return start, end, output


def neighbours(point) -> list[tuple[int, int]]:
    (r, c) = point
    return [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]


def recreate_path(
    end: tuple[int, int], closed_set: PMap[tuple[int, int], tuple[int, int] | None]
) -> PVector[tuple[int, int]]:
    path = pvector([end])
    while True:
        prev = closed_set[path[-1]]
        if prev is None:
            return pvector(reversed(path))
        else:
            path = path.append(prev)


def shortest_path(
    start: tuple[int, int],
    h: Callable[[tuple[int, int]], int],
    exclusion: Callable[[tuple[int, int], tuple[int, int]], bool],
    finish: Callable[[tuple[int, int]], bool],
    field: PMap[tuple[int, int], int],
) -> PVector[tuple[int, int]] | None:

    open_set = {start}
    closed_set: PMap[tuple[int, int], tuple[int, int] | None] = pmap(
        {start: None}, pre_size=len(field)
    )
    g_score = {start: 0}
    f_score = {start: h(start)}
    while len(open_set) > 0:
        current = min(open_set, key=lambda x: f_score[x])
        open_set.discard(current)
        if finish(current):
            return recreate_path(current, closed_set)
        for neighbour in neighbours(current):
            if not exclusion(neighbour, current):
                test_score = g_score[current] + 1
                if g_score.get(neighbour, float("inf")) > test_score:
                    closed_set = closed_set.set(neighbour, current)
                    g_score[neighbour] = test_score
                    f_score[neighbour] = test_score + h(neighbour)
                    open_set.add(neighbour)


def display_path(path: PVector[tuple[int, int]]) -> str:
    rows = max(row for (row, _) in path) + 1
    cols = max(col for (_, col) in path) + 1
    paths = [["."] * cols for _ in range(rows)]

    def get_char(a, b) -> str:
        (r1, c1) = a
        (r2, c2) = b
        match (r2 - r1, c2 - c1):
            case (0, 1):
                return ">"
            case (0, -1):
                return "<"
            case (1, 0):
                return "V"
            case (-1, 0):
                return "^"

    for (p, n) in zip(path, path[1:]):
        paths[p[0]][p[1]] = get_char(p, n)
    paths[path[-1][0]][path[-1][1]] = "E"
    return "\n".join("".join(r) for r in paths)


if __name__ == "__main__":
    with open("../inputs/day_12") as f:
        start, end, field = parse_input(f.read())
        sp = shortest_path(
            start,
            lambda p: abs(p[0] - end[0]) + abs(p[1] - end[1]),
            lambda n, c: n not in field or (field[n] - field[c] > 1),
            lambda p: p == end,
            field,
        )
        print(f"Part 1: {len(sp[:-1])}")
        finish_set = {k for (k, v) in field.items() if v == 0}
        p2_sp = shortest_path(
            end,
            lambda _: 0,
            lambda n, c: n not in field or (field[n] - field[c] < -1),
            lambda p: p in finish_set,
            field,
        )
        print(f"Part 1: {len(p2_sp[:-1])}")

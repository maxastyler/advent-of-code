import re
from typing import Iterator


def parse(input: str) -> list[int | None]:
    return [
        None if noop == "noop" else int(addx)
        for (noop, addx) in re.findall("(noop)|addx (-?\d+)", input)
    ]


def cpu_state(instructions: Iterator[None | int]) -> int:
    state = 1
    for inst in instructions:
        match inst:
            case None:
                yield state
            case x:
                yield state
                yield state
                state = state + x
    yield state


def part_1(instructions: list[int | None]) -> int:
    cpu = cpu_state(input)
    for _ in range(19):
        cpu.__next__()
    return sum((i + 20) * v for (i, v) in enumerate(cpu) if i % 40 == 0)


def part_2(instructions: list[int | None]) -> str:
    return "".join(
        "#" if abs(c - x) <= 1 else "." + ("\n" if x == 39 else "")
        for (c, x) in zip(cpu_state(instructions), (pixel % 40 for pixel in range(240)))
    )


if __name__ == "__main__":
    with open("../inputs/day_10") as f:
        input = parse(f.read())
        print(f"Part 1: {part_1(input)}")
        print(f"Part 2:\n{part_2(input)}")

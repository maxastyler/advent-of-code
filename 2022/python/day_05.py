import itertools as it
import re


def parse_crates(crate_string: str) -> dict[int, list[str]]:
    row_regex = re.compile("\s*(.*)(\d)")
    return {
        int(g[2]): list(reversed(g[1]))
        for col in it.zip_longest(
            *[list(i) for i in crate_string.split("\n")], fillvalue=" "
        )
        if (g := row_regex.match("".join(col))) is not None
    }

def parse_instructions(instructions: str) -> list[tuple[int, int, int]]:
    row_regex = re.compile("move (\d+) from (\d+) to (\d+)")
    return [
        tuple(int(i) for i in row_regex.match(row).groups())
        for row in instructions.split("\n")
    ]


def run_instructions(
    crates: dict[int, list[str]], instructions: list[tuple[int, int, int]]
):
    for (num, i, j) in instructions:
        for _ in range(num):
            crates[j].append(crates[i].pop())


def run_instructions_ordered(
    crates: dict[int, list[str]], instructions: list[tuple[int, int, int]]
):
    for (num, i, j) in instructions:
        crates[j].extend(crates[i][-num:])
        crates[i] = crates[i][:-num]


with open("../inputs/day_05") as f:
    [p1, p2] = f.read().split("\n\n")
    crates_p1 = parse_crates(p1)
    crates_p2 = parse_crates(p1)
    run_instructions(crates_p1, parse_instructions(p2))
    run_instructions_ordered(crates_p2, parse_instructions(p2))
    print("Part 1: ", "".join(crates_p1[k][-1] for k in sorted(crates_p1.keys())))
    print("Part 2: ", "".join(crates_p2[k][-1] for k in sorted(crates_p2.keys())))

import operator
from math import lcm
from typing import Callable, Self

from attr import frozen, field, evolve
from pyrsistent import pdeque, pmap
from pyrsistent.typing import PDeque, PVector, PMap
import re

items_reg = re.compile("(\d+)")


@frozen
class Monkey:
    items: PDeque[int] = field()
    interest_op: Callable[[int], int] = field()
    division: int = field()
    true_monkey: int = field()
    false_monkey: int = field()

    @classmethod
    def create(cls, data: tuple[str, str, str, str, str, str]) -> Self:
        items, op, var, division, true_monkey, false_monkey = data

        def bind_interest_fun():
            bound_op = operator.mul if op == "*" else operator.add
            if var == "old":
                return lambda old: bound_op(old, old)
            else:
                return lambda old: bound_op(old, int(var))

        return Monkey(
            items=pdeque(int(i) for i in items_reg.findall(items)),
            interest_op=bind_interest_fun(),
            division=int(division),
            true_monkey=int(true_monkey),
            false_monkey=int(false_monkey),
        )

    def next_monkey(self, worry: int) -> int:
        if worry % self.division == 0:
            return self.true_monkey
        else:
            return self.false_monkey


def parse_input(input: str) -> PMap[int, Monkey]:
    monkeys = re.findall(
        "Monkey (\d+):\s*"
        "Starting items:\s*([\d, ]+)\s*"
        "Operation: new = old ([*+]) (old|\d+)\s*"
        "Test: divisible by (\d+)\s*"
        "If true: throw to monkey (\d+)\s*"
        "If false: throw to monkey (\d+)",
        input,
    )

    return pmap({int(monkey): Monkey.create(rest) for (monkey, *rest) in monkeys})


def turn(
    monkeys: PMap[int, Monkey], current_monkey: int, modulo: int, *, part_1: bool
) -> PMap[int, Monkey]:
    m = monkeys[current_monkey]
    if part_1:
        next_worry = m.interest_op(m.items.left) // 3
    else:
        next_worry = m.interest_op(m.items.left)
    next_worry = next_worry % modulo
    next_monkey_key = m.next_monkey(next_worry)
    next_monkey = monkeys[next_monkey_key]
    return monkeys.set(current_monkey, evolve(m, items=m.items.popleft())).set(
        next_monkey_key, evolve(next_monkey, items=next_monkey.items.append(next_worry))
    )


def round(
    monkeys: PMap[int, Monkey], modulo: int, *, part_1: bool
) -> tuple[PMap[int, Monkey], dict[int, int]]:
    inspections = {k: 0 for k in monkeys}
    current_monkeys = monkeys
    for key in sorted(monkeys.keys()):
        while len(current_monkeys[key].items) > 0:
            inspections[key] += 1
            current_monkeys = turn(current_monkeys, key, modulo, part_1=part_1)

    return current_monkeys, inspections


def monkey_business(
    monkeys: PMap[int, Monkey], modulo: int, part_1: bool, num_turns: int
) -> int:
    inspections = {k: 0 for k in monkeys}
    for _ in range(num_turns):
        monkeys, new_inspections = round(monkeys, modulo, part_1=part_1)
        for k in inspections:
            inspections[k] += new_inspections[k]
    [a, b] = sorted(inspections.values(), reverse=True)[:2]
    return a * b


def part_1(monkeys: PMap[int, Monkey], modulo: int) -> int:
    return monkey_business(monkeys, modulo, part_1=True, num_turns=20)


def part_2(monkeys: PMap[int, Monkey], modulo: int) -> int:
    return monkey_business(monkeys, modulo, part_1=False, num_turns=10000)


if __name__ == "__main__":
    with open("../inputs/day_11") as f:
        monkeys = parse_input(f.read())
        modulo = lcm(*[m.division for m in monkeys.values()])
        print(f"Part 1: {part_1(monkeys, modulo)}")
        print(f"Part 2: {part_2(monkeys, modulo)}")

import itertools
import re
from typing import Self

from attrs import frozen, field, evolve
from pyrsistent import pvector, pset, pmap, m, freeze
from pyrsistent.typing import PVector, PSet, PMap


@frozen
class PuzzleState:
    positions: PVector[tuple[str, int]] = field()
    time_left: int = field()
    turned_on: PSet[str] = field()
    flow: int = field()
    released: int = field()

    def next_action(
        self, graph: PMap[str, tuple[int, PMap[str, int]]]
    ) -> Self | tuple[Self, PVector[str]]:
        if self.time_left <= 0:
            return Self
        elif len(graph) == len(self.turned_on):
            return evolve(
                self, released=self.released + self.flow * self.time_left, time_left=0
            )
        else:
            min_t = min(v for (_, v) in self.positions)
            if min_t >= self.time_left:
                return evolve(
                    self,
                    released=self.released + self.flow * self.time_left,
                    time_left=0,
                )
            min_keys = pvector(k for (k, v) in self.positions if v == min_t)
            bigger_positions = pvector(
                (k, v - min_t) for (k, v) in self.positions if v > min_t
            )
            return (
                PuzzleState(
                    positions=bigger_positions,
                    time_left=self.time_left - min_t,
                    turned_on=self.turned_on.union(min_keys),
                    flow=self.flow
                    + sum(
                        graph[i][0] for i in set(min_keys) if i not in self.turned_on
                    ),
                    released=self.released + self.flow * min_t,
                ),
                min_keys,
            )

    def get_best_score(self, graph: PMap[str, tuple[int, PMap[str, int]]]) -> int:
        next_action = self.next_action(graph)
        if isinstance(next_action, PuzzleState):
            return next_action.released
        else:
            (state, positions) = next_action
            max_score = 0
            for keys in itertools.product(*[graph[i][1].items() for i in positions]):
                if any(t+1 < state.time_left for (k, t) in keys):
                    max_score = max(max_score, evolve(state, positions = state.positions.extend(pvector(keys))).get_best_score(graph))
            return max_score


def parse_input(input: str) -> dict[str, tuple[int, set[str]]]:
    str_reg = re.compile("([A-Z]+)")
    return {
        e1: (int(flow), set(str_reg.findall(e2s)))
        for (e1, flow, e2s) in re.findall(
            "Valve (\D+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)",
            input,
        )
    }


def bfs(start: str, graph: dict[str, tuple[int, set[str]]]) -> dict[str, str | None]:
    g_score = {start: 0}
    parents = {start: None}
    next = [start]
    while len(next) > 0:
        current = next.pop()
        test_score = g_score[current] + 1
        for neighbour in graph[current][1]:
            if g_score.get(neighbour, float("inf")) > test_score:
                next.append(neighbour)
                g_score[neighbour] = test_score
                parents[neighbour] = current
    return parents


def length(parents: dict[str, str | None], node: str) -> int:
    current = node
    length = 0
    while True:
        if parents[current] == None:
            return length
        current = parents[current]
        length += 1


def neighbour_lengths(
    key: str, graph: dict[str, tuple[int, set[str]]], important: list[str]
) -> dict[str, int]:
    parents = bfs(key, graph)
    return {k: length(parents, k) for k in important if k != key}


def shorten_paths(
    graph: dict[str, tuple[int, set[str]]]
) -> dict[str, tuple[int, dict[str, int]]]:
    important = ["AA"] + [k for (k, (n, _)) in graph.items() if n > 0]
    return {
        k: (i, neighbour_lengths(k, graph, important)) for (k, (i, _)) in graph.items()
    }


def best_choice(
    graph: dict[str, tuple[int, dict[str, int]]],
    turned_on: set[str],
    position: str = "AA",
    per_minute: int = 0,
    time_left: int = 30,
    total_released: int = 0,
) -> tuple[int, list[str]]:
    if time_left <= 0:
        return total_released, [position]
    elif len(turned_on) == len(graph):
        return total_released + time_left * per_minute, [position]
    maximum = 0
    max_list = []

    for (pos, dist) in graph[position][1].items():
        if (time_left - (dist + 1) >= 1) and (pos not in turned_on):
            (test_max, new_list) = best_choice(
                graph,
                turned_on | {pos},
                pos,
                per_minute + graph[pos][0],
                time_left - (dist + 1),
                total_released + (per_minute * (dist + 1)),
            )
            if test_max > maximum:
                maximum = test_max
                max_list = new_list
    if maximum == 0:
        return total_released + time_left * per_minute, [position]
    else:
        return maximum, [position] + max_list


def split(state: list[tuple[str, int]]) -> tuple[set[str], list[tuple[str, int]]]:
    timed_out = set()
    rest = []
    for (k, t) in state:
        if t <= 0:
            timed_out.add(k)
        else:
            rest.append((k, t))
    return timed_out, rest


if __name__ == "__main__":
    with open("../inputs/day_16") as f:
        inp = parse_input(f.read())

        graph = shorten_paths(inp)
        print(best_choice(graph, set()))
        # print(
        #     PuzzleState(pvector([("AA", 3), ("AA", 2)]), 26, pset(), 0, 0).get_best_score(freeze(graph))
        # )

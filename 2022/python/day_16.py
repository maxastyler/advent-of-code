import re
from typing import Iterable
import itertools as it

from attrs import frozen, field, evolve
from pyrsistent import pvector, pset, pmap, m, freeze
from pyrsistent.typing import PVector, PSet, PMap


@frozen
class Agent:
    time_left: int = field()
    position: str = field()


def split_agents(
    agents: Iterable[Agent],
) -> tuple[int, PVector[Agent], PVector[Agent]]:
    """Splits the agents into (need task, have task)"""
    min_time = min(a.time_left for a in agents)
    return (
        min_time,
        pvector(
            evolve(a, time_left=0) for a in agents if a.time_left == min_time
        ),
        pvector(
            evolve(a, time_left=a.time_left - min_time)
            for a in agents
            if a.time_left != min_time
        ),
    )


@frozen
class PuzzleState:
    agents: PVector[Agent] = field()
    time_left: int = field()
    turned_on: PSet[str] = field()
    flow: int = field()
    released: int = field()

    def score(self, graph: PMap[str, tuple[int, PMap[str, int]]]) -> int:
        if self.time_left <= 0:
            return self.released
        (agent_time, ready, active) = split_agents(self.agents)
        if agent_time >= self.time_left or len(self.turned_on) == len(graph):
            return self.released + (self.flow * self.time_left)
        time_left = self.time_left - agent_time
        turned_on = self.turned_on
        flow = self.flow
        for a in ready:
            if a.position not in turned_on:
                flow += graph[a.position][0]
                turned_on = turned_on.add(a.position)

        targets = [k for k in graph.keys() if k not in turned_on]
        max_score = 0
        for new_positions in it.product(*([targets] * len(ready))):
            if any(
                (graph[a.position][1][pos] + 1 < time_left)
                for a, pos in zip(ready, new_positions)
            ):
                active = active.extend(
                    Agent(graph[a.position][1][pos], pos)
                    for a, pos in zip(ready, new_positions)
                )
                max_score = max(
                    PuzzleState(
                        agents=active,
                        time_left=time_left,
                        turned_on=turned_on,
                        flow=flow,
                        released=self.released,
                    ).score(graph),
                    max_score,
                )
        if max_score == 0:
            return self.released + (self.flow * self.time_left)
        else:
            return max_score


# @frozen
# class PuzzleState:
#     positions: PVector[tuple[str, int]] = field()
#     time_left: int = field()
#     turned_on: PSet[str] = field()
#     flow: int = field()
#     released: int = field()

#     def next_action(
#         self, graph: PMap[str, tuple[int, PMap[str, int]]]
#     ) -> "PuzzleState" | tuple["PuzzleState", PVector[str]]:
#         if self.time_left <= 0:
#             return self
#         elif len(graph) == len(self.turned_on):
#             return evolve(
#                 self,
#                 released=self.released + self.flow * self.time_left,
#                 time_left=0,
#             )
#         else:
#             min_t = min(v for (_, v) in self.positions)
#             if min_t >= self.time_left:
#                 return evolve(
#                     self,
#                     released=self.released + self.flow * self.time_left,
#                     time_left=0,
#                 )
#             min_keys = pvector(k for (k, v) in self.positions if v == min_t)
#             bigger_positions = pvector(
#                 (k, v - min_t) for (k, v) in self.positions if v > min_t
#             )
#             return (
#                 PuzzleState(
#                     positions=bigger_positions,
#                     time_left=self.time_left - min_t,
#                     turned_on=self.turned_on.union(min_keys),
#                     flow=self.flow
#                     + sum(
#                         graph[i][0]
#                         for i in set(min_keys)
#                         if i not in self.turned_on
#                     ),
#                     released=self.released + self.flow * min_t,
#                 ),
#                 min_keys,
#             )

#     def get_best_score(
#         self, graph: PMap[str, tuple[int, PMap[str, int]]]
#     ) -> int:
#         next_action = self.next_action(graph)
#         if isinstance(next_action, PuzzleState):
#             return next_action.released
#         else:
#             (state, positions) = next_action
#             max_score = 0
#             for keys in itertools.product(
#                 *[graph[i][1].items() for i in positions]
#             ):
#                 if any(t + 1 < state.time_left for (k, t) in keys):
#                     max_score = max(
#                         max_score,
#                         evolve(
#                             state,
#                             positions=state.positions.extend(pvector(keys)),
#                         ).get_best_score(graph),
#                     )
#             return max_score


def parse_input(input: str) -> dict[str, tuple[int, set[str]]]:
    str_reg = re.compile("([A-Z]+)")
    return {
        e1: (int(flow), set(str_reg.findall(e2s)))
        for (e1, flow, e2s) in re.findall(
            "Valve (\D+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)",
            input,
        )
    }


def bfs(
    start: str, graph: dict[str, tuple[int, set[str]]]
) -> dict[str, str | None]:
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
        k: (i, neighbour_lengths(k, graph, important))
        for (k, (i, _)) in graph.items()
        if k in important
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


def split(
    state: list[tuple[str, int]]
) -> tuple[set[str], list[tuple[str, int]]]:
    timed_out = set()
    rest = []
    for (k, t) in state:
        if t <= 0:
            timed_out.add(k)
        else:
            rest.append((k, t))
    return timed_out, rest


with open("../inputs/day_16") as f:
    inp = parse_input(f.read())
    graph = shorten_paths(inp)
    pstate = PuzzleState(
        agents=pvector([Agent(0, "AA")]),
        time_left=30,
        turned_on=pset("AA"),
        flow=0,
        released=0,
    )
    pstate.score(graph)
    # print(best_choice(graph, set()))
    # pstate.score(graph)
    # print(
    #     PuzzleState(pvector([("AA", 3), ("AA", 2)]), 26, pset(), 0, 0).get_best_score(freeze(graph))
    # )

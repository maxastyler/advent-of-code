import re


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

def best_choice_p2(
        graph: dict[str, tuple[int, dict[str, int]]],
        turned_on: set[str],
        state: list[tuple[str, int]],
        per_minute: int = 0,
        time_left: int = 26,
        total_released: int = 0,
) -> int:



def best_choice_p2(
    graph: dict[str, tuple[int, dict[str, int]]],
    turned_on: set[str],
    state: list[tuple[str, int]],
    per_minute: int = 0,
    time_left: int = 26,
    total_released: int = 0,
) -> int:
    if time_left <= 0:
        return total_released
    elif len(turned_on) == len(graph):
        return total_released + time_left * per_minute
    maximum = 0

    to_move = [i for (i, t) in state if t <= 0]
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
        return total_released + time_left * per_minute
    else:
        return maximum

if __name__ == "__main__":
    with open("../inputs/day_16") as f:
        inp = parse_input(f.read())

        graph = shorten_paths(inp)
        print(best_choice(graph, set()))

from functools import reduce


def priority(letter: str) -> int:
    return (
        ord(letter) - ord("A") + 27 if letter.isupper() else ord(letter) - ord("a") + 1
    )


def wrong_item(bag: str) -> set[str]:
    return set(bag[: len(bag) // 2]).intersection(bag[len(bag) // 2 :])


def find_badge(group: list[str]) -> set[str]:
    return reduce(lambda acc, v: set(acc).intersection(v), group)


with open("../inputs/day_03") as f:
    input = list(f.read().splitlines())
    print(sum(priority(item) for bag in map(wrong_item, input) for item in bag))
    print(
        sum(
            priority(badge)
            for i in range(len(input) // 3)
            for badge in find_badge(input[i * 3 : i * 3 + 3])
        )
    )

def score(play: tuple[str, str]) -> int:
    play_value = ord(play[1]) - ord('X')
    match (play_value - (ord(play[0]) - ord('A'))) % 3:
        case 0: return 3 + play_value + 1
        case 1: return 6 + play_value + 1
        case _: return play_value + 1

def score_2(play: tuple[str, str]) -> int:
    other_score = ord(play[0]) - ord('A')
    match play[1]:
        case 'X': return ((other_score - 1) % 3) + 1
        case 'Y': return other_score + 4
        case 'Z': return ((other_score + 1) % 3) + 7


with open("../inputs/day_02") as f:
    plays = [tuple(l.split()) for l in f.readlines()]
    print(f"Part 1: {sum(score(p) for p in plays)}")
    print(f"Part 2: {sum(score_2(p) for p in plays)}")


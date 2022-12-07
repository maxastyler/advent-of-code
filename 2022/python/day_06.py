def find_unique_pos(string: str, window_size: int) -> int:
    for i in range(len(data)-window_size):
        if len(set(string[i:i+window_size])) == window_size:
            return i+window_size
    return -1

with open("../inputs/day_06") as f:
    data = f.read().strip()
    print(f"Part 1: {find_unique_pos(data, 4)}")
    print(f"Part 2: {find_unique_pos(data, 14)}")

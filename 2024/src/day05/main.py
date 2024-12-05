from functools import cmp_to_key
from pathlib import Path


def split_rules(rule: str):
    return list(map(int, rule.split("|")))


def split_updates(update: str):
    return list(map(int, update.split(",")))


def passes_rules(rules: list[list[int]], before: int, after: int):
    for rule in rules:
        # If in wrong order
        if rule[1] == before and rule[0] == after:
            return False
    return True


def is_valid_update(rules: list[list[int]], update: list[int]):
    for i in range(len(update)):
        target = update[i]
        before = update[:i]
        after = update[i + 1 :]
        for b in before:
            if not passes_rules(rules, b, target):
                return False
        for a in after:
            if not passes_rules(rules, target, a):
                return False
    return True


def part1(name: str):
    file = open(Path(__file__).parent / name, "r").read()

    rules, updates = file.split("\n\n")

    rules = list(map(split_rules, rules.strip().split("\n")))
    updates = list(map(split_updates, updates.strip().split("\n")))

    valid_updates = []

    for update in updates:
        if is_valid_update(rules, update):
            valid_updates.append(update)

    total = 0

    for update in valid_updates:
        update_len = len(update)
        total += update[int((update_len / 2) - 0.5)]

    return total


def sorted_update(rules: list[list[int]], invalid: list[int]):
    def sort_num(a: int, b: int):
        a_i = invalid.index(a)
        b_i = invalid.index(b)

        if a_i > b_i:
            if not passes_rules(rules, b, a):
                return -1
        if a_i < b_i:
            if not passes_rules(rules, a, b):
                return 1
        return 0

    return sorted(invalid, key=cmp_to_key(sort_num))


def part2(name: str):
    file = open(Path(__file__).parent / name, "r").read()

    rules, updates = file.split("\n\n")

    rules = list(map(split_rules, rules.strip().split("\n")))
    updates = list(map(split_updates, updates.strip().split("\n")))

    invalid_updates = []

    for update in updates:
        if not is_valid_update(rules, update):
            invalid_updates.append(update)

    sorted_invalid_updates = []

    for invalid in invalid_updates:
        sorted_invalid_updates.append(sorted_update(rules, invalid))

    total = 0

    for update in sorted_invalid_updates:
        update_len = len(update)
        total += update[int((update_len / 2) - 0.5)]

    return total


def test_sample_p1():
    assert part1("sample.txt") == 143


def test_p1():
    assert part1("input.txt") == 5166


def test_sample_p2():
    assert part2("sample.txt") == 123


def test_p2():
    assert part2("input.txt") == 4679


if __name__ == "__main__":
    print(part2("input.txt"))

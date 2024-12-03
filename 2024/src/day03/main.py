import re
from pathlib import Path


def part1(name: str):
    input = open(Path(__file__).parent / name, "r").read()

    matches = re.findall(r"mul\(\d{1,3},\d{1,3}\)", input)

    total = 0

    for match in matches:
        left, right = match.split(",")
        left = int(left[4:])
        right = int(right[:-1])
        total += left * right

    return total


def part2(name: str):
    input = open(Path(__file__).parent / name, "r").read()

    matches = re.findall(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)", input)

    total = 0
    enabled = True

    for match in matches:
        if match == "do()":
            enabled = True
            continue
        if match == "don't()":
            enabled = False
            continue

        if not enabled:
            continue

        left, right = match.split(",")
        left = int(left[4:])
        right = int(right[:-1])
        total += left * right

    return total


def test_sample_p1():
    assert part1("sample.txt") == 161


def test_p1():
    assert part1("input.txt") == 175700056


def test_sample_p2():
    assert part2("sample2.txt") == 48


def test_p2():
    assert part2("input.txt") == 71668682


if __name__ == "__main__":
    print(part2("input.txt"))

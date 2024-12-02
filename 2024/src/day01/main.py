from collections import Counter
from pathlib import Path


def parse(name: str):
    lines = open(Path(__file__).parent / name, "r").readlines()

    aArr: list[int] = []
    bArr: list[int] = []

    for line in lines:
        a, b = line.split()
        aArr.append(int(a))
        bArr.append(int(b))

    return aArr, bArr


def part1(name: str):
    aArr, bArr = parse(name)

    aArr.sort()
    bArr.sort()

    total = 0

    for i in range(len(bArr)):
        a = aArr[i]
        b = bArr[i]
        total += abs(b - a)

    return total


def part2(name: str):
    aArr, bArr = parse(name)

    total = 0
    cnt = Counter(bArr)

    for i in range(len(aArr)):
        a = aArr[i]
        total += cnt[a] * a

    return total


def test_sample():
    assert part1("sample.txt") == 11


def test_p1():
    assert part1("input.txt") == 1660292


def test_p2():
    assert part2("input.txt") == 22776016


if __name__ == "__main__":
    print(part1("sample.txt"))

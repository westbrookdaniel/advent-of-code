from collections import Counter
from pathlib import Path


def main_p1(name: str):
    lines = open(Path(__file__).parent / name, "r").readlines()

    aArr: list[int] = []
    bArr: list[int] = []

    for line in lines:
        a, b = line.split()
        aArr.append(int(a))
        bArr.append(int(b))

    aArr.sort()
    bArr.sort()

    total = 0

    for i in range(len(bArr)):
        a = aArr[i]
        b = bArr[i]
        total += abs(b - a)

    return total


def main_p2(name: str):
    lines = open(Path(__file__).parent / name, "r").readlines()

    aArr: list[int] = []
    bArr: list[int] = []

    for line in lines:
        a, b = line.split()
        aArr.append(int(a))
        bArr.append(int(b))

    total = 0
    cnt = Counter(bArr)

    for i in range(len(aArr)):
        a = aArr[i]
        total += cnt[a] * a

    return total


def test_sample():
    assert main_p1("sample.txt") == 11


def test_p1():
    assert main_p1("p1.txt") == 1660292


def test_p2():
    assert main_p2("p1.txt") == 22776016


if __name__ == "__main__":
    print(main_p1("sample.txt"))

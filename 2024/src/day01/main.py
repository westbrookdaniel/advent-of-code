from pathlib import Path


def main_p1(name: str):
    file = open(Path(__file__).parent / name, "r").read()
    lines = file.strip().split("\n")

    aArr: list[int] = []
    bArr: list[int] = []

    for line in lines:
        a, b = line.split("  ")
        aArr.append(int(a))
        bArr.append(int(b))

    aArr.sort()
    bArr.sort()

    total = 0

    for i in range(len(bArr)):
        a = aArr[i]
        b = bArr[i]
        if b > a:
            total += b - a
        else:
            total += a - b

    return total


def main_p2(name: str):
    file = open(Path(__file__).parent / name, "r").read()
    lines = file.strip().split("\n")

    aArr: list[int] = []
    bArr: list[int] = []

    for line in lines:
        a, b = line.split("  ")
        aArr.append(int(a))
        bArr.append(int(b))

    total = 0

    for i in range(len(aArr)):
        a = aArr[i]

        found = 0

        for j in range(len(bArr)):
            b = bArr[j]
            if b == a:
                found += 1

        total += found * a

    return total


def test_sample():
    assert main_p1("sample.txt") == 11


def test_p1():
    assert main_p1("p1.txt") == 1660292


if __name__ == "__main__":
    print(main_p2("p1.txt"))

from pathlib import Path


def main(name: str):
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


def test_sample():
    assert main("sample.txt") == 11


if __name__ == "__main__":
    print(main("p1.txt"))

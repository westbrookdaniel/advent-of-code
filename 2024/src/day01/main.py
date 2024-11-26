from pathlib import Path


def main(name: str):
    file = open(Path(__file__).parent / name, "r").read()
    lines = file.strip().split("\n")

    total = 0

    for line in lines:
        first = None
        last = None
        for c in line:
            if c.isdigit():
                last = c
                if first is None:
                    first = c

        if first and last:
            total += int(first + last)

    return total


def test_sample():
    assert main("sample.txt") == 142


if __name__ == "__main__":
    print(main("sample.txt"))

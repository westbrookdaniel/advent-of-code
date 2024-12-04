from pathlib import Path


def get(grid: list[list[str]], r: int, c: int):
    if r < 0:
        return ""
    if c < 0:
        return ""
    if r + 1 > len(grid):
        return ""
    if c + 1 > len(grid[r]):
        return ""
    return grid[r][c]


def matches_part1(*chars):
    if "".join(chars) == "XMAS":
        return 1
    return 0


def matches_part2(*chars):
    if "".join(chars) == "AMSSM":
        return 1
    if "".join(chars) == "ASSMM":
        return 1
    if "".join(chars) == "ASMMS":
        return 1
    if "".join(chars) == "AMMSS":
        return 1
    return 0


def part1(name: str):
    file = open(Path(__file__).parent / name, "r").read()
    lines = file.strip().split("\n")

    grid = list(map(list, lines))

    total = 0

    for r in range(len(grid)):
        row = grid[r]
        for c in range(len(row)):
            if row[c] != "X":
                continue
            for r_dir in range(-1, 2):
                for c_dir in range(-1, 2):
                    r2 = r + r_dir * 1
                    r3 = r + r_dir * 2
                    r4 = r + r_dir * 3
                    c2 = c + c_dir * 1
                    c3 = c + c_dir * 2
                    c4 = c + c_dir * 3
                    total += matches_part1(
                        get(grid, r, c),
                        get(grid, r2, c2),
                        get(grid, r3, c3),
                        get(grid, r4, c4),
                    )

    return total


def part2(name: str):
    file = open(Path(__file__).parent / name, "r").read()
    lines = file.strip().split("\n")

    grid = list(map(list, lines))

    total = 0

    for r in range(len(grid)):
        row = grid[r]
        for c in range(len(row)):
            if row[c] != "A":
                continue
            total += matches_part2(
                get(grid, r, c),
                get(grid, r - 1, c - 1),
                get(grid, r - 1, c + 1),
                get(grid, r + 1, c + 1),
                get(grid, r + 1, c - 1),
            )

    return total


def test_sample_p1():
    assert part1("sample.txt") == 18


def test_p1():
    assert part1("input.txt") == 2569


def test_sample_p2():
    assert part2("sample.txt") == 9


def test_p2():
    assert part2("input.txt") == 1998


if __name__ == "__main__":
    print(part2("input.txt"))

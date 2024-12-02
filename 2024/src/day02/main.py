from pathlib import Path


def part1(name: str):
    lines = open(Path(__file__).parent / name, "r").readlines()

    safe = 0
    for report in lines:
        levels = list(map(int, report.split()))
        if not is_unsafe(levels):
            safe += 1

    return safe


def part2(name: str):
    lines = open(Path(__file__).parent / name, "r").readlines()

    safe = 0
    for report in lines:
        levels = list(map(int, report.split()))
        if is_any_safe(levels):
            safe += 1

    return safe


def is_any_safe(levels: list[int]):
    if not is_unsafe(levels):
        return True

    for i in range(len(levels)):
        removed = levels.copy()
        del removed[i]
        if not is_unsafe(removed):
            return True

    return False


def is_unsafe(levels: list[int]):
    isIncr: None | bool = None
    prev: int | None = None

    for level in levels:
        if prev is None:
            prev = level
            continue

        if isIncr is None:
            isIncr = prev < level

        diff = abs(prev - level)

        if diff < 1 or diff > 3:
            return True

        if isIncr:
            if prev > level:
                return True
        else:
            if prev < level:
                return True

        prev = level

    return False


def test_sample_p1():
    assert part1("sample.txt") == 2


def test_p1():
    assert part1("input.txt") == 559


def test_sample_p2():
    assert part2("sample.txt") == 4


def test_p2():
    assert part2("input.txt") == 601


if __name__ == "__main__":
    print(part2("input.txt"))

from pathlib import Path


def part1(name: str):
    lines = open(Path(__file__).parent / name, "r").readlines()

    safe = 0

    for report in lines:
        levels = map(int, report.split())
        prev: int | None = None
        isIncr: bool | None = None
        unsafe = False

        for level in levels:
            if prev is None:
                prev = level
            else:
                if isIncr is None:
                    isIncr = prev < level

                diff = abs(prev - level)

                if diff < 1 or diff > 3:
                    unsafe = True

                if isIncr:
                    if prev > level:
                        unsafe = True
                else:
                    if prev < level:
                        unsafe = True

                prev = level

        if not unsafe:
            safe += 1

    return safe


def part2(name: str):
    lines = open(Path(__file__).parent / name, "r").readlines()

    safe = 0

    for report in lines:
        if check_report(report):
            safe += 1

    return safe


# true if safe
def check_report(report):
    levels = list(map(int, report.split()))

    if not check(levels):
        return True

    for i in range(len(levels)):
        removed = levels.copy()
        del removed[i]
        if not check(removed):
            return True

    return False


# true if unsafe (yeah ik)
def check(levels):
    isIncr: None | bool = None
    prev: int | None = None

    for level in levels:
        if prev is None:
            prev = level
        else:
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

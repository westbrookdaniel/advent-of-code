from pathlib import Path


def chunks(lst, n):
    """Yield successive n-sized chunks from lst."""
    for i in range(0, len(lst), n):
        yield lst[i : i + n]


def create_disk(file: str):
    # Find new format for storing disk that supports higher index numbers

    chars = list(file.strip())

    infos = list(chunks(chars, 2))
    disk: list[int | None] = []

    for id in range(len(infos)):
        info = infos[id]
        blocks, space = map(int, (info + [0])[:2])
        for _ in range(blocks):
            disk.append(id)
        for _ in range(space):
            disk.append(None)

    return disk


def compact(disk: list[int | None]):
    i = 0
    while i < len(disk):
        space = disk[i]
        if space is None:
            last = disk.pop()
            if last is None:
                continue
            disk[i] = last
        i += 1

    return disk


def part1(name: str):
    file = open(Path(__file__).parent / name, "r").read()

    disk = create_disk(file)
    compacted = compact(disk)

    sum = 0
    for i in range(len(list(compacted))):
        n = compacted[i]
        if n is None:
            continue
        sum += i * n

    return sum


def test_sample_p1():
    assert part1("sample.txt") == 1928


# def test_p1():
#     assert part1("input.txt") == 5166


# def test_sample_p2():
#     assert part2("sample.txt") == 123


# def test_p2():
#     assert part2("input.txt") == 4679


if __name__ == "__main__":
    print("")
    print(part1("input.txt"))

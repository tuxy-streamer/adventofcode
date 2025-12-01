def read_file(path: str) -> list[str]:
    file = open(path, "r")
    line = [line.strip() for line in file]
    file.close()
    return line


def first_half(input: list[str]) -> int:
    counter: int = 0
    current_position = 50
    for string in input:
        direction: str = string[:1]
        distance: int = int(string[1:])
        if direction == "L":
            current_position = (current_position - distance) % 100
        else:
            current_position = (current_position + distance) % 100
        if current_position == 0:
            counter += 1
    return counter


def second_half(input: list[str]) -> int:
    counter = 0
    current_position = 50
    for string in input:
        direction, distance = string[0], int(string[1:])
        step = -1 if direction == "L" else 1
        for _ in range(distance):
            current_position = (current_position + step) % 100
            if current_position == 0:
                counter += 1

    return counter


def day1() -> None:
    test: list[str] = [
        "L68",
        "L30",
        "R48",
        "L5",
        "R60",
        "L55",
        "L1",
        "L99",
        "R14",
        "L82",
    ]
    content = read_file("day1.txt")

    assert first_half(test) == 3, "Test input answer is given as 3"
    print(first_half(content))

    assert second_half(test) == 6, "Test input answer is given as 6"
    print(second_half(content))

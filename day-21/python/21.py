#!/usr/bin/env python3

from __future__ import annotations
from collections import deque
from enum import Enum
from typing import TypeAlias

ACTUAL_STEPS = 26_501_365


class Tile(Enum):
    Garden = 1
    Rock = 2

    @staticmethod
    def parse(c: str):
        match c:
            case "." | "S":
                return Tile.Garden
            case "#":
                return Tile.Rock
            case _:
                raise ValueError(f"invalid tile: {c}")

    def __repr__(self):
        match self:
            case Tile.Garden:
                return "."
            case Tile.Rock:
                return "#"


class Point:
    def __init__(self, y: int, x: int) -> None:
        self.y = y
        self.x = x

    def __repr__(self):
        return f"({self.y}, {self.x})"

    def as_tuple(self):
        return (self.y, self.x)

    def __eq__(self, other):
        return self.as_tuple() == other.as_tuple()

    def __hash__(self):
        return hash(self.as_tuple())

    def up(self) -> Point | None:
        if self.y == 0:
            return None
        else:
            return Point(self.y - 1, self.x)

    def down(self, height: int) -> Point | None:
        if self.y >= height - 1:
            return None
        else:
            return Point(self.y + 1, self.x)

    def left(self) -> Point | None:
        if self.x == 0:
            return None
        else:
            return Point(self.y, self.x - 1)

    def right(self, width: int) -> Point | None:
        if self.x >= width - 1:
            return None
        else:
            return Point(self.y, self.x + 1)

    def reachable_neigbors(self, grid: Grid) -> list[Point]:
        return [
            point
            for point in [
                self.up(),
                self.down(grid.height()),
                self.left(),
                self.right(grid.width()),
            ]
            if point is not None and grid.lookup(point) == Tile.Garden
        ]


class Grid:
    def __init__(self, input: str) -> None:
        self.grid = [list(map(Tile.parse, line)) for line in input.splitlines()]

    def __repr__(self) -> str:
        return "\n".join(["".join(map(repr, row)) for row in self.grid])

    def height(self) -> int:
        return len(self.grid)

    def width(self) -> int:
        return len(self.grid[0])

    def lookup(self, point: Point) -> Tile:
        return self.grid[point.y][point.x]

    def distances_for_subgrid(self, y: int, x: int) -> dict[Point, int]:
        match (signum(y), signum(x)):
            case (-1, -1):
                start = (Point(self.height() - 1, self.width() - 1), 2)
            case (-1, 0):
                start = (Point(self.height() - 1, self.width() // 2), 1)
            case (-1, 1):
                start = (Point(self.height() - 1, 0), 2)
            case (0, -1):
                start = (Point(self.height() // 2, self.width() - 1), 1)
            case (0, 0):
                start = (Point(self.height() // 2, self.width() // 2), 0)
            case (0, 1):
                start = (Point(self.height() // 2, 0), 1)
            case (1, -1):
                start = (Point(0, self.width() - 1), 2)
            case (1, 0):
                start = (Point(0, self.width() // 2), 1)
            case (1, 1):
                start = (Point(0, 0), 2)
            case _:
                raise ValueError("invalid signum")

        reachable_tiles: dict[Point, int] = {}
        queue: deque[QueueItem] = deque([start])

        while len(queue) > 0:
            (point, distance) = queue.popleft()
            if point not in reachable_tiles:
                reachable_tiles[point] = distance
                queue.extend(
                    [
                        (neighbor, distance + 1)
                        for neighbor in point.reachable_neigbors(self)
                    ]
                )

        return reachable_tiles

    def distance_to_subgrid(self, gy: int, gx: int) -> int:
        if gy == 0:
            if gx == 0:
                return 0  # starting subgrid
            else:
                return self.width() // 2 + self.width() * (abs(gx) - 1)
        else:
            if gx == 0:
                return self.height() // 2 + self.height() * (abs(gy) - 1)
            else:
                return (
                    self.width() // 2
                    + self.height() // 2
                    + self.width() * (abs(gx) - 1)
                    + self.height() * (abs(gy) - 1)
                )

    def num_reachable_in_subgrid(self, gy: int, gx: int) -> int:
        distance_to_here = self.distance_to_subgrid(gy, gx)

        distances = map(
            lambda x: x[1] + distance_to_here,
            grid.distances_for_subgrid(gy, gx).items(),
        )
        return len(
            [
                distance
                for distance in distances
                if distance <= ACTUAL_STEPS and distance % 2 == 1
            ]
        )


QueueItem: TypeAlias = tuple[Point, int]


def signum(x: int) -> int:
    if x < 0:
        return -1
    elif x > 0:
        return 1
    else:
        return 0


def part1(grid: Grid):
    part1 = len(
        [
            distance
            for (_, distance) in grid.distances_for_subgrid(0, 0).items()
            if distance % 2 == 0 and distance <= 64
        ]
    )

    print(f"part1 = {part1}")


def part2(grid: Grid):
    x = (ACTUAL_STEPS - grid.width() // 2) // grid.width()
    y = (ACTUAL_STEPS - grid.height() // 2) // grid.height()
    assert x == y

    up_cap = grid.num_reachable_in_subgrid(-y, 0)
    down_cap = grid.num_reachable_in_subgrid(y, 0)
    left_cap = grid.num_reachable_in_subgrid(0, -x)
    right_cap = grid.num_reachable_in_subgrid(0, x)

    up_left_small_diag = grid.num_reachable_in_subgrid(-y, -1)
    up_left_big_diag = grid.num_reachable_in_subgrid(-y + 1, -1)
    up_right_small_diag = grid.num_reachable_in_subgrid(-y, 1)
    up_right_big_diag = grid.num_reachable_in_subgrid(-y + 1, 1)
    down_left_small_diag = grid.num_reachable_in_subgrid(1, -x)
    down_left_big_diag = grid.num_reachable_in_subgrid(1, -x + 1)
    down_right_small_diag = grid.num_reachable_in_subgrid(1, x)
    down_right_big_diag = grid.num_reachable_in_subgrid(1, x - 1)

    small_diag_segments = y
    big_diag_segments = small_diag_segments - 1

    caps = up_cap + down_cap + left_cap + right_cap
    big_diagonals = big_diag_segments * (
        up_left_big_diag + up_right_big_diag + down_left_big_diag + down_right_big_diag
    )
    small_diagonals = small_diag_segments * (
        up_left_small_diag
        + up_right_small_diag
        + down_left_small_diag
        + down_right_small_diag
    )
    diagonals = big_diagonals + small_diagonals

    inner_odd = grid.num_reachable_in_subgrid(0, 0)
    inner_even = grid.num_reachable_in_subgrid(0, 1)

    inner_total = inner_odd
    ring = 1

    while ring < x:  # or y
        if ring % 2 == 0:
            per_subgrid = inner_odd
        else:
            per_subgrid = inner_even

        inner_total += ring * 4 * per_subgrid
        ring += 1

    part2 = caps + diagonals + inner_total
    print(f"part2 = {part2}")


if __name__ == "__main__":
    # with open("../test_input.txt", "r") as f:
    with open("../input.txt", "r") as f:
        input = f.read().rstrip()

    grid = Grid(input)
    part1(grid)
    part2(grid)

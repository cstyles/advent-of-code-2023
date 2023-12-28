#!/usr/bin/env python3

from __future__ import annotations
from collections import deque
from enum import Enum
from typing import TypeAlias


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


if __name__ == "__main__":
    # with open("../test_input.txt", "r") as f:
    with open("../input.txt", "r") as f:
        input = f.read().rstrip()

    grid = Grid(input)
    part1(grid)

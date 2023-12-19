import std.array;
import std.stdio;
import std.file;
import std.range;
import std.string;
import std.utf;
import std.typecons;

import std.algorithm.iteration;
import std.typecons;
import std.container.binaryheap;

void main()
{
	/* string input = readText("../test_input.txt"); */
	string input = readText("../input.txt");

	immutable uint[][] grid = input
		.splitLines
		.map!(line => line.map!(c => c - '0').array)
		.array;

	/* writeln(grid); */

	solve!(false)(grid).writeln;
}

uint solve(bool part_two)(immutable uint[][] grid)
{
	uint moves_remaining;
	if (part_two)
	{
		moves_remaining = 10;
	}
	else
	{
		moves_remaining = 3;
	}

	immutable ulong height = grid.length;
	immutable ulong width = grid[0].length;

	HeapItem start = HeapItem(Point(0, 0), Direction.Right, moves_remaining, 0);

	immutable Point destination = Point(height - 1, width - 1);
	uint best_so_far = uint.max;
	int[MapKey] shortest_distances;

	// Min-heap
	auto heap = heapify!"a > b"([start]);

	// TODO: part_two => insert down start

	while (!heap.empty)
	{
		auto heap_item = heap.front;
		heap.popFront;

		if (heap_item.total_distance > best_so_far)
		{
			continue;
		}

		// TODO: part 2: check moves_remaining
		if (heap_item.point == destination)
		{
			// TODO: set best_so_far
			return heap_item.total_distance;
		}

		auto for_map = heap_item.for_map;
		if (for_map in shortest_distances)
		{
			auto in_map = shortest_distances[for_map];
			if (heap_item.total_distance >= in_map)
			{
				continue;
			}
			else
			{
				shortest_distances[for_map] = heap_item.total_distance;
			}
		}
		else
		{
			shortest_distances[for_map] = heap_item.total_distance;
		}

		HeapItem[] next = heap_item.possible_moves!(part_two)(grid);
		foreach (HeapItem next_item; next) {
			heap.insert(next_item);
		}

		// TODO: possible_moves
	}

	return 0; // TODO
}

struct Point
{
	ulong y;
	ulong x;

	this(ulong y, ulong x)
	{
		this.y = y;
		this.x = x;
	}

	Nullable!Point up()
	{
		if (this.y == 0)
		{
			return Nullable!Point.init;
		}
		else
		{
			return Nullable!Point(Point(this.y - 1, this.x));
		}
	}

	Nullable!Point down(ulong height)
	{
		if (this.y >= height - 1)
		{
			return Nullable!Point.init;
		}
		else
		{
			return Nullable!Point(Point(this.y + 1, this.x));
		}
	}

	Nullable!Point left()
	{
		if (this.x == 0)
		{
			return Nullable!Point.init;
		}
		else
		{
			return Nullable!Point(Point(this.y, this.x - 1));
		}
	}

	Nullable!Point right(ulong width)
	{
		if (this.x >= width - 1)
		{
			return Nullable!Point.init;
		}
		else
		{
			return Nullable!Point(Point(this.y, this.x + 1));
		}
	}

	Nullable!Point in_direction(Direction direction, immutable uint[][] grid)
	{
		switch (direction)
		{
		case Direction.Up:
			return this.up;
		case Direction.Down:
			return this.down(grid.length);
		case Direction.Left:
			return this.left;
		case Direction.Right:
			return this.right(grid[0].length);
		default:
			return Nullable!Point.init;
		}
	}

	uint lookup(immutable uint[][] grid)
	{
		return grid[this.y][this.x];
	}
}

enum Direction
{
	Up,
	Down,
	Left,
	Right
}

struct HeapItem
{
	Point point;
	Direction direction;
	uint moves_remaining;
	uint total_distance;

	MapKey for_map()
	{
		return MapKey(this.point, this.direction, this.moves_remaining);
	}

	int opCmp(const HeapItem other) const
	{
		int td_cmp = this.total_distance.cmp(other.total_distance);
		if (td_cmp == 0)
		{
			return this.moves_remaining.cmp(other.moves_remaining);
		}
		else
		{
			return td_cmp;
		}
	}

	HeapItem[] possible_moves(bool part_two)(immutable uint[][] grid)
	{
		// TODO: part2 and moves_remaining

		HeapItem[] moves = [];

		Direction[] directions;
		switch (this.direction)
		{
		case Direction.Up:
			directions = [Direction.Up, Direction.Left, Direction.Right];
			break;
		case Direction.Down:
			directions = [Direction.Down, Direction.Left, Direction.Right];
			break;
		case Direction.Left:
			directions = [Direction.Up, Direction.Left, Direction.Down];
			break;
		case Direction.Right:
			directions = [Direction.Up, Direction.Down, Direction.Right];
			break;
		default:
			return []; // unreachable
		}

		foreach (Direction direction; directions)
		{
			if (direction == this.direction)
			{
				if (this.moves_remaining == 0)
				{
					continue;
				}
				else
				{
					Nullable!Point new_point = this.point.in_direction(direction, grid);
					if (!new_point.isNull)
					{
						Point point = new_point.get;
						moves ~= HeapItem(point, direction, this.moves_remaining - 1, this.total_distance + point.lookup(
								grid));
					}
				}
			}
			else
			{
				// TODO: part 2 moves_remaining
				Nullable!Point new_point = this.point.in_direction(direction, grid);
				if (!new_point.isNull)
				{
					Point point = new_point.get;
					moves ~= HeapItem(point, direction, 2, this.total_distance + point.lookup(
							grid));
				}
			}
		}

		return moves;
	}
}

alias MapKey = Tuple!(Point, "point", Direction, "direction", uint, "moves_remaining");

int cmp(uint a, uint b)
{
	if (a > b)
	{
		return 1;
	}
	else if (a < b)
	{
		return -1;
	}
	else
	{
		return 0;
	}
}

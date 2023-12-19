import std.stdio;
import std.file;
import std.range;
import std.string;
import std.typecons;
import std.algorithm.iteration;
import std.container.binaryheap;

void main()
{
	/* string input = readText("../test_input.txt"); */
	string input = readText("../input.txt");

	immutable uint[][] grid = input
		.splitLines
		.map!(line => line.map!(c => c - '0').array)
		.array;

	uint part1 = solve!(false)(grid);
	uint part2 = solve!(true)(grid);

	writefln("part1 = %s", part1);
	writefln("part2 = %s", part2);
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
	int[MapKey] shortest_distances;

	// Min-heap
	auto heap = heapify!"a > b"([start]);

	if (part_two)
	{
		HeapItem start_down = start;
		start_down.moves_remaining = 0;
		heap.insert(start_down);
	}

	while (!heap.empty)
	{
		auto heap_item = heap.front;
		heap.popFront;

		if (heap_item.point == destination && (!part_two || heap_item.moves_remaining <= 6))
		{
			return heap_item.total_distance;
		}

		immutable MapKey for_map = heap_item.for_map;
		if (for_map in shortest_distances)
		{
			immutable auto in_map = shortest_distances[for_map];
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

		HeapItem[] possible_moves = heap_item.possible_moves!(part_two)(grid);
		foreach (immutable HeapItem next_item; possible_moves)
		{
			heap.insert(next_item);
		}
	}

	assert(0, "didn't reach the destination :(");
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
			assert(0, "unreachable");
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
		immutable int td_cmp = this.total_distance.cmp(other.total_distance);
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
		HeapItem[] moves = [];

		if (part_two && this.moves_remaining > 6)
		{
			immutable Nullable!Point new_point = this.point.in_direction(this.direction, grid);
			if (!new_point.isNull)
			{
				Point actual_point = new_point.get;
				moves ~= HeapItem(actual_point, direction, this.moves_remaining - 1, this.total_distance + actual_point.lookup(
						grid));
			}

			return moves;
		}

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
			assert(0, "unreachable");
		}

		foreach (immutable Direction direction; directions)
		{
			if (direction == this.direction)
			{
				if (this.moves_remaining == 0)
				{
					continue;
				}
				else
				{
					immutable Nullable!Point new_point = this.point.in_direction(direction, grid);
					if (!new_point.isNull)
					{
						Point actual_point = new_point.get;
						moves ~= HeapItem(actual_point, direction, this.moves_remaining - 1, this.total_distance + actual_point
								.lookup(
									grid));
					}
				}
			}
			else
			{
				uint new_moves_remaining;
				if (part_two)
				{
					new_moves_remaining = 9;
				}
				else
				{
					new_moves_remaining = 2;
				}

				immutable Nullable!Point new_point = this.point.in_direction(direction, grid);
				if (!new_point.isNull)
				{
					Point actual_point = new_point.get;
					moves ~= HeapItem(actual_point, direction, new_moves_remaining, this.total_distance + actual_point.lookup(
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
	return a - b;
}

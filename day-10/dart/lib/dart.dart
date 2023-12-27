import 'dart:math';

class Grid {
  List<List<Tile>> grid = [];

  Grid(String input) {
    final lines = input.split('\n');

    for (var y = 0; y < lines.length; y++) {
      var row = <Tile>[];
      final line = lines[y];

      for (var x = 0; x < line.length; x++) {
        row.add(parseTile(line[x]));
      }

      grid.add(row);
    }
  }

  @override
  String toString() {
    return grid
        .map((row) => row.map((tile) => tile.toString()).join())
        .join('\n');
  }

  int height() {
    return grid.length;
  }

  int width() {
    return grid[0].length;
  }

  Point start() {
    for (var y = 0; y < height(); y++) {
      for (var x = 0; x < width(); x++) {
        if (grid[y][x] == Tile.start) {
          return Point(y, x);
        }
      }
    }

    throw 'unreachable';
  }

  Tile lookup(Point point) {
    return grid[point.y][point.x];
  }

  void removeSuperfluousTiles(Set<Point> pipe) {
    for (var y = 0; y < height(); y++) {
      for (var x = 0; x < width(); x++) {
        if (!pipe.contains(Point(y, x))) {
          grid[y][x] = Tile.ground;
        }
      }
    }
  }

  Tile calculateStart(Point start) {
    final up = [Tile.vertical, Tile.southwest, Tile.southeast]
        .contains(lookup(start.up()!));
    final down = [Tile.vertical, Tile.northwest, Tile.northeast]
        .contains(lookup(start.down(height())!));
    final left = [Tile.horizontal, Tile.northeast, Tile.southeast]
        .contains(lookup(start.left()!));
    final right = [Tile.horizontal, Tile.northwest, Tile.southwest]
        .contains(lookup(start.right(width())!));

    switch ((up, down, left, right)) {
      case (true, true, false, false):
        return Tile.vertical;
      case (true, false, true, false):
        return Tile.northwest;
      case (true, false, false, true):
        return Tile.northeast;
      case (false, true, true, false):
        return Tile.southwest;
      case (false, true, false, true):
        return Tile.southeast;
      case (false, false, true, true):
        return Tile.horizontal;
      default:
        throw 'unreachable :(';
    }
  }
}

enum Tile {
  vertical,
  horizontal,
  northeast,
  northwest,
  southwest,
  southeast,
  ground,
  start,
}

Tile parseTile(String char) {
  switch (char) {
    case '|':
      return Tile.vertical;
    case '-':
      return Tile.horizontal;
    case 'L':
      return Tile.northeast;
    case 'J':
      return Tile.northwest;
    case '7':
      return Tile.southwest;
    case 'F':
      return Tile.southeast;
    case '.':
      return Tile.ground;
    case 'S':
      return Tile.start;
    default:
      throw 'invalid char';
  }
}

class Point {
  int y = 0;
  int x = 0;

  Point(this.y, this.x);

  @override
  String toString() => "($y, $x)";

  @override
  operator ==(other) => other is Point && other.x == x && other.y == y;

  @override
  int get hashCode => Object.hash(x, y);

  Point? up() {
    if (y == 0) {
      return null;
    } else {
      return Point(y - 1, x);
    }
  }

  Point? down(int height) {
    if (y >= height - 1) {
      return null;
    } else {
      return Point(y + 1, x);
    }
  }

  Point? left() {
    if (x == 0) {
      return null;
    } else {
      return Point(y, x - 1);
    }
  }

  Point? right(int width) {
    if (x >= width - 1) {
      return null;
    } else {
      return Point(y, x + 1);
    }
  }

  Iterable<Point> neighbors(Grid grid) {
    return [up(), down(grid.height()), left(), right(grid.width())]
        .whereType<Point>();
  }

  Iterable<Point> reachableNeighbors(Grid grid) =>
      neighbors(grid).where((neighbor) => reachable(neighbor, grid));

  bool reachable(Point other, Grid grid) {
    final direction = diff(other);
    final thisTile = grid.lookup(this);
    final otherTile = grid.lookup(other);

    switch (direction) {
      case Direction.up:
        return [Tile.vertical, Tile.northeast, Tile.northwest, Tile.start]
                .contains(thisTile) &&
            [Tile.vertical, Tile.southeast, Tile.southwest, Tile.start]
                .contains(otherTile);
      case Direction.down:
        return [Tile.vertical, Tile.southeast, Tile.southwest, Tile.start]
                .contains(thisTile) &&
            [Tile.vertical, Tile.northeast, Tile.northwest, Tile.start]
                .contains(otherTile);
      case Direction.left:
        return [Tile.horizontal, Tile.northwest, Tile.southwest, Tile.start]
                .contains(thisTile) &&
            [Tile.horizontal, Tile.northeast, Tile.southeast, Tile.start]
                .contains(otherTile);
      case Direction.right:
        return [Tile.horizontal, Tile.northeast, Tile.southeast, Tile.start]
                .contains(thisTile) &&
            [Tile.horizontal, Tile.northwest, Tile.southwest, Tile.start]
                .contains(otherTile);
      default:
        return false;
    }
  }

  /// Returns the direction to move in to go from self to other.
  ///
  /// Assumes the points are neighbors.
  Direction diff(Point other) {
    if (y < other.y) {
      return Direction.down;
    } else if (y > other.y) {
      return Direction.up;
    } else if (x < other.x) {
      return Direction.right;
    } else if (x > other.x) {
      return Direction.left;
    } else {
      throw 'bad diff';
    }
  }
}

Set<Point> part1(Grid grid, Point start) {
  final routes = start.reachableNeighbors(grid);
  var distances = {start: 0};
  var pipe = <Point>{start};

  for (var route in routes) {
    distances[route] = 1;
    var distance = 1;
    var cameFrom = start;
    while (route != start) {
      final next = route
          .reachableNeighbors(grid)
          .firstWhere((element) => element != cameFrom);

      distance += 1;
      var d = distances[next] ?? distance;
      distances[next] = min(d, distance);

      pipe.add(route);
      cameFrom = route;
      route = next;
    }
  }

  final part1 = distances.values.reduce(max);
  print('part1 = $part1');

  return pipe;
}

void part2(Grid grid, Set<Point> pipe, Point start) {
  grid.removeSuperfluousTiles(pipe);
  grid.grid[start.y][start.x] = grid.calculateStart(start);

  var inside = <Point>{};
  for (var y = 0; y < grid.height(); y++) {
    for (var x = 0; x < grid.width(); x++) {
      final start = Point(y, x);
      if (pipe.contains(start)) {
        continue;
      }

      var crossings = 0;
      Direction? lastCross;
      Point? point = start;

      while (point != null) {
        final tile = grid.lookup(point);

        if (tile == Tile.horizontal) {
          crossings += 1;
          lastCross = null;
        } else if ([Tile.northwest, Tile.southwest].contains(tile) &&
            lastCross == null) {
          lastCross = Direction.left;
        } else if ([Tile.northwest, Tile.southwest].contains(tile) &&
            lastCross == Direction.right) {
          crossings += 1;
          lastCross = null;
        } else if ([Tile.northeast, Tile.southeast].contains(tile) &&
            lastCross == null) {
          lastCross = Direction.right;
        } else if ([Tile.northeast, Tile.southeast].contains(tile) &&
            lastCross == Direction.left) {
          crossings += 1;
          lastCross = null;
        } else if (tile == Tile.northwest && lastCross == Direction.left) {
          lastCross = null;
        } else if (tile == Tile.northeast && lastCross == Direction.right) {
          lastCross = null;
        }

        point = point.down(grid.height());
      }

      if (crossings % 2 == 1) {
        inside.add(start);
      }
    }
  }

  final part2 = inside.length;
  print('part2 = $part2');
}

enum Direction {
  up,
  down,
  left,
  right,
}

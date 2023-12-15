import Foundation

enum Tile {
  case ash
  case rock

  static func parse(char: Character) -> Tile {
    switch char {
    case "#": Tile.rock
    case ".": Tile.ash
    default: fatalError("bad tile: '\(char)'")
    }
  }
}

func width(_ pattern: [[Tile]]) -> Int {
  pattern[0].count
}

// Transpose an Array.
func columns(_ pattern: [[Tile]]) -> [[Tile]] {
  var columns: [[Tile]] = []
  let height = pattern.count
  let width = width(pattern)

  for x in 0..<width {
    var column: [Tile] = []
    for y in 0..<height {
      column.append(pattern[y][x])
    }

    columns.append(column)
  }

  return columns
}

func vertical_reflection(_ pattern: [[Tile]]) -> Int? {
  let columns = columns(pattern)

  for x in 1..<columns.count {
    var left = x - 1
    var right = x

    while columns[left] == columns[right] {
      if left == 0 || right == columns.count - 1 {
        return x
      } else {
        left -= 1
        right += 1
      }
    }
  }

  return nil
}

func horizontal_reflection(_ pattern: [[Tile]]) -> Int? {
  for y in 1..<pattern.count {
    var upper = y - 1
    var lower = y

    while pattern[lower] == pattern[upper] {
      if upper == 0 || lower == pattern.count - 1 {
        return y
      } else {
        upper -= 1
        lower += 1
      }
    }
  }

  return nil
}

func summarize(_ pattern: [[Tile]]) -> Int {
  if let vertical_reflection = vertical_reflection(pattern) {
    return vertical_reflection
  } else {
    return 100 * horizontal_reflection(pattern)!
  }
}

let file_name = "../test_input.txt"
// let file_name = "../input.txt"

func parse_pattern(string: Substring) -> [[Tile]] {
  string
    .split(separator: "\n")
    .map({ line in line.map(Tile.parse) })
}

let input = try String(contentsOfFile: file_name)
let patterns = input.split(separator: "\n\n").map(parse_pattern)

let part1 = patterns.map(summarize).reduce(0, +)
print("part1 = \(part1)")

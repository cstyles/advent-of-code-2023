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

func vertical_reflection_smudge(_ pattern: [[Tile]]) -> Int? {
  let columns = columns(pattern)
  let old_vertical_reflection = vertical_reflection(pattern)

  for x in 1..<columns.count {
    if old_vertical_reflection == x {
      continue
    }

    var left = x - 1
    var right = x
    var smudge_fixed = false

    while true {
      if columns[left] != columns[right] {
        if !smudge_fixed && one_off(columns[left], columns[right]) {
          smudge_fixed = true
        } else {
          break
        }
      }

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

func horizontal_reflection_smudge(_ pattern: [[Tile]]) -> Int? {
  let old_horizontal_reflection = horizontal_reflection(pattern)

  for y in 1..<pattern.count {
    if old_horizontal_reflection == y {
      continue
    }

    var upper = y - 1
    var lower = y
    var smudge_fixed = false

    while true {
      if pattern[lower] != pattern[upper] {
        if !smudge_fixed && one_off(pattern[lower], pattern[upper]) {
          smudge_fixed = true
        } else {
          break
        }
      }

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

func summarize_smudge(_ pattern: [[Tile]]) -> Int {
  if let vertical_reflection = vertical_reflection_smudge(pattern) {
    return vertical_reflection
  } else {
    return 100 * horizontal_reflection_smudge(pattern)!
  }
}

func one_off(_ a: [Tile], _ b: [Tile]) -> Bool {
  zip(a, b).filter({ (a, b) in a != b }).count == 1
}

// let file_name = "../test_input.txt"
let file_name = "../input.txt"

func parse_pattern(string: Substring) -> [[Tile]] {
  string
    .split(separator: "\n")
    .map({ line in line.map(Tile.parse) })
}

let input = try String(contentsOfFile: file_name)
let patterns = input.split(separator: "\n\n").map(parse_pattern)

let part1 = patterns.map(summarize).reduce(0, +)
print("part1 = \(part1)")

let part2 = patterns.map(summarize_smudge).reduce(0, +)
print("part2 = \(part2)")

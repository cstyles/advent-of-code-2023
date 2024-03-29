require "./point"

struct Glyph # "Symbol" is a native Crystal class
  property character, point

  def self.try_new(character, point)
    case character
    when '.', '0'..'9'
      nil
    else
      new character, point
    end
  end

  def initialize(@character : Char, @point : Point); end
end

struct PartNumber
  property value

  def initialize(@point : Point, @value : Int32); end
end

def main
  input = File.read "../input.txt"
  grid = input.lines.map { |line| line.chars }

  glyphs = input
    .lines
    .each_with_index
    .flat_map { |line, y| [y].cycle.zip line.chars.each_with_index }
    .compact_map { |(y, (c, x))| Glyph.try_new(c, Point.new(y, x)) }
    .to_a

  part_numbers = glyphs
    .flat_map { |glyph| glyph.point.neighbors }
    .compact_map { |neighbor| find_number grid, neighbor }
    .to_set

  part1 = part_numbers.map { |num| num.value }.sum
  puts "part1 = #{part1}"

  part2 = glyphs
    .select { |glyph| glyph.character == '*' }
    .compact_map { |glyph| gear_ratio grid, glyph.point }
    .sum

  puts "part2 = #{part2}"
end

def lookup(grid, point)
  grid[point.y][point.x]
end

def find_number(grid, point)
  digit = lookup(grid, point).to_digit
  return if digit.nil?
  digits = [digit]

  # Try parsing to the right
  digits_to_the_right = Unfold.new(point, &.right)
    .map_while { |point| lookup(grid, point).to_digit }

  digits.concat digits_to_the_right

  # Try parsing to the left
  digits_to_the_left = Unfold.new(point, &.left)
    .map_while { |point| lookup(grid, point).to_digit }
    .each do |digit|
      digits.unshift digit
      point = point.left.not_nil!
    end

  value = parse_number digits
  PartNumber.new point, value
end

def parse_number(digits)
  digits.reduce(0) { |acc, elm| acc * 10 + elm }
end

def gear_ratio(grid, point)
  neighbors = point
    .neighbors
    .compact_map { |neighbor| find_number grid, neighbor }
    .to_set

  if neighbors.size == 2
    neighbors.map(&.value).product
  else
    nil
  end
end

main

class Unfold(T)
  include Iterator(T)

  def initialize(@x : T, &block : T -> (T | Nil))
    @f = block
  end

  def next
    if temp = @f.call @x
      @x = temp
    else
      stop
    end
  end
end

struct Char
  # Basically `to_i` but returns `nil` on failure instead of raising.
  def to_digit
    to_i
  rescue
    nil
  end
end

module Iterator(T)
  def map_while(&func : T -> U | Nil) forall U
    MapWhile(typeof(self), T, U).new(self, func)
  end
end

class MapWhile(I, T, U)
  include Iterator(U)
  include IteratorWrapper

  def initialize(@iterator : I, @func : T -> U | Nil)
    @done = false
  end

  def next
    return stop if @done
    value = wrapped_next.try &@func

    if value.nil?
      @done = true
      stop
    else
      value
    end
  end
end

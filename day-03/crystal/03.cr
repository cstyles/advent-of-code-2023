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

  glyphs = [] of Glyph
  input.lines.each_with_index do |line, y|
    line.chars.each_with_index do |c, x|
      point = Point.new y, x
      glyph = Glyph.try_new c, point
      glyphs << glyph unless glyph.nil?
    end
  end

  part_numbers = glyphs
    .flat_map { |glyph| glyph.point.neighbors }
    .compact_map { |neighbor| find_number grid, neighbor }
    .to_set

  part1 = part_numbers.map { |num| num.value }.sum
  puts "part1 = #{part1}"
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
    .map { |point| lookup(grid, point).to_digit }
    .take_while { |digit| !digit.nil? }
    .map { |digit| digit.not_nil! } # no map_wnile?

  digits.concat digits_to_the_right
  digits

  # Try parsing to the left
  digits_to_the_left = Unfold.new(point, &.left)
    .map { |point| lookup(grid, point).to_digit }
    .take_while { |digit| !digit.nil? }
    .map { |digit| digit.not_nil! } # TODO: no map_wnile? :(
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

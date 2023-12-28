#!/usr/bin/env ruby
# frozen_string_literal: true

Point = Data.define(:x, :y, :z) do
  def self.parse(string)
    x, y, z = string.split ', '
    new Float(x), Float(y), Float(z)
  end
end

Velocity = Data.define(:x, :y, :z) do
  def self.parse(string)
    x, y, z = string.split ', '
    new Float(x), Float(y), Float(z)
  end

  def xy_slope
    y / x
  end
end

Hailstone = Data.define(:position, :velocity) do
  def self.parse(line)
    position, velocity = line.split ' @ '
    position = Point.parse position
    velocity = Velocity.parse velocity

    new position, velocity
  end

  def xy_intersect(other)
    if xy_slope == other.xy_slope
      nil # parallel
    else
      x = (other.b - b) / (xy_slope - other.xy_slope)
      Point.new x, y(x), 0
    end
  end

  def y(x)
    xy_slope * x + b
  end

  def xy_slope
    velocity.xy_slope
  end

  def b
    position.y - position.x * xy_slope
  end

  def in_future?(point)
    case [position.x <=> point.x, velocity.x.negative?]
    in [-1, false]
      true
    in [1, true]
      true
    else
      false
    end
  end
end

def find_xy_intersections(hailstones, range)
  intersections = []

  hailstones.each_with_index do |a, i|
    hailstones[i + 1..].each do |b|
      intersection = a.xy_intersect b
      next if intersection.nil?

      next unless range.include? intersection.x
      next unless range.include? intersection.y
      next unless a.in_future? intersection
      next unless b.in_future? intersection

      intersections << intersection
    end
  end

  intersections
end

def main
  # input = File.readlines('../test_input.txt').map(&:rstrip)
  # range = 7.0..27.0
  input = File.readlines('../input.txt').map(&:rstrip)
  range = 200_000_000_000_000.0..400_000_000_000_000.0

  hailstones = input.map { Hailstone.parse _1 }
  part1 = find_xy_intersections(hailstones, range).length
  puts "part1 = #{part1}"
end

main

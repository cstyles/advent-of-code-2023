class Instruction
  new: (direction, magnitude) =>
    @direction = direction
    @magnitude = magnitude

parse_instruction1 = (line) ->
  chars = string.gmatch(line, "%w+")
  direction = chars()
  magnitude = tonumber(chars())
  Instruction(direction, magnitude)

parse_direction2 = (num) ->
  switch num
    when '0'
      'R'
    when '1'
      'D'
    when '2'
      'L'
    when '3'
      'U'

parse_instruction2 = (line) ->
  color = string.gmatch(line, "%x%x%x%x%x%x")()
  direction = string.sub(color, 6, 6)
  direction = parse_direction2(direction)
  magnitude = tonumber(string.sub(color, 1, 5), 16)
  Instruction(direction, magnitude)

class Point
  new: (y, x) =>
    @y = y
    @x = x

  move_by: (instruction) =>
    switch (instruction.direction)
      when 'U'
        Point(@y - instruction.magnitude, @x)
      when 'D'
        Point(@y + instruction.magnitude, @x)
      when 'L'
        Point(@y, @x - instruction.magnitude)
      when 'R'
        Point(@y, @x + instruction.magnitude)

input = io.lines "../input.txt"
instructions1 = [parse_instruction1(line) for line in input]
input = io.lines "../input.txt"
instructions2 = [parse_instruction2(line) for line in input]

solve = (instructions) ->
  point = Point(0, 0)
  vertices = { point }
  perimeter = 0

  for _, instruction in ipairs instructions
    point = point\move_by(instruction)
    table.insert(vertices, point)
    perimeter += instruction.magnitude

  area = 0
  for i = 1, #vertices - 1
    a = vertices[i]
    b = vertices[i + 1]
    area += (a.y + b.y) * (a.x - b.x)

  result = area // 2 + perimeter // 2 + 1
  print(result)

solve(instructions1)
solve(instructions2)

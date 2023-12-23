#!/usr/bin/env deno run --allow-read

class Range {
  start: number;
  end: number;

  static from_pair(a: string, b: string): Range {
    const start = parseInt(a);
    const end = parseInt(b) + 1;

    return new Range(start, end);
  }

  constructor(start: number, end: number) {
    this.start = start;
    this.end = end;
  }

  down(): Range {
    return new Range(this.start - 1, this.end - 1);
  }

  clone(): Range {
    return new Range(this.start, this.end);
  }

  contains(num: number): boolean {
    return num >= this.start && num < this.end;
  }

  overlap(other: Range): Range {
    const start = Math.max(this.start, other.start);
    const end = Math.min(this.end, other.end);

    return new Range(start, end);
  }

  is_empty(): boolean {
    return this.start >= this.end;
  }
}

class Brick {
  x: Range;
  y: Range;
  z: Range;

  constructor(x: Range, y: Range, z: Range) {
    this.x = x;
    this.y = y;
    this.z = z;
  }

  static parse(string: string): Brick {
    const [start, end] = string.split("~");

    // No zip function :(
    const start_items = start.split(",");
    const end_items = end.split(",");

    const x = Range.from_pair(start_items[0], end_items[0]);
    const y = Range.from_pair(start_items[1], end_items[1]);
    const z = Range.from_pair(start_items[2], end_items[2]);

    return new Brick(x, y, z);
  }

  down(): Brick {
    return new Brick(this.x.clone(), this.y.clone(), this.z.down());
  }

  intersects(other: Brick): boolean {
    return !(this.x.overlap(other.x).is_empty() ||
      this.y.overlap(other.y).is_empty() ||
      this.z.overlap(other.z).is_empty());
  }

  // clone(): Brick {
  //   return new Brick(this.x.clone(), this.y.clone(), this.z.clone());
  // }
}

// const input = await Deno.readTextFile("../test_input.txt");
const input = await Deno.readTextFile("../input.txt");

const bricks = input.trimEnd().split("\n").map((line) => Brick.parse(line));
bricks.sort((a, b) => a.z.start - b.z.start);

for (let i = 0; i < bricks.length; i++) {
  while (true) {
    const this_brick = bricks[i];
    if (this_brick.z.contains(1)) {
      break;
    }

    const down = this_brick.down();
    if (bricks.slice(0, i).some((brick) => brick.intersects(down))) {
      break;
    } else {
      bricks[i] = down;
    }
  }
}

// console.log(bricks);

const supports: Map<number, Set<number>> = new Map();

for (let i = bricks.length - 1; i >= 0; i--) {
  const this_brick = bricks[i];
  const down = this_brick.down();
  const supports_for_this: Set<number> = new Set();

  for (let j = 0; j < i; j++) {
    if (down.intersects(bricks[j])) {
      supports_for_this.add(j);
    }
  }

  supports.set(i, supports_for_this);
}

// console.log(supports);

const cant_disintegrate: Set<number> = new Set();
for (const sups of supports.values()) {
  if (sups.size == 1) {
    for (const i of sups) {
      cant_disintegrate.add(i);
    }
  }
}

// console.log(cant_disintegrate);

const part1 = bricks.length - cant_disintegrate.size;
console.log(`part1 = ${part1}`);

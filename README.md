# Advent of Code 2023

## Bird's eye view:

| Day  | Language      | Solution                                              | Runtime (ms) | Rust solution                      | Rust runtime (ms) |
| ---- | ------------- | ----------------------------------------------------- | ------------ | ---------------------------------- | ------------------|
| [1]  | [Uiua]        | [01.ua](day-01/uiua/01.ua)                            | 330          | [day-01](day-01/rust/src/main.rs)  | 1.4               |
| [2]  | [Fish shell]  | [01.fish](day-02/fish/02.fish)                        | 146          | [day-02](day-02/rust/src/main.rs)  | 1.1               |
| [3]  | [Crystal]     | [03.cr](day-03/crystal/03.cr)                         | 90           | [day-03](day-03/rust/src/main.rs)  | 1.5               |
| [4]  | [Koka]        | [04.kk](day-04/koka/04.kk)                            | 5.6          | [day-04](day-04/rust/src/main.rs)  | 1.0               |
| [5]  | [OCaml]       | [05.ml](day-05/ocaml/05.ml)                           | 2.5          | [day-05](day-05/rust/src/main.rs)  | 1.2               |
| [6]  | [Pen & Paper] | [06.jpg](day-06/pen-and-paper/06.jpg)                 | N/A          | [day-06](day-06/rust/src/main.rs)  | 0.8               |
| [7]  | [Clojure]     | [07.clj](day-07/clojure/07.clj)                       | 794          | [day-07](day-07/rust/src/main.rs)  | 7.3               |
| [8]  | [Inko]        | [08.inko](day-08/inko/08.inko)                        | 175          | [day-08](day-08/rust/src/main.rs)  | 13                |
| [9]  | [SML]         | [09.sml](day-09/sml/09.sml)                           | 17.3         | [day-09](day-09/rust/src/main.rs)  | 1.4               |
| [10] | [Dart]        | [dart.dart](day-10/dart/bin/dart.dart)                | 43.4         | [day-10](day-10/rust/src/main.rs)  | 6.2               |
| [11] | [Fennel]      | [11.fnl](day-11/fennel/11.fnl)                        | 369          | [day-11](day-11/rust/src/main.rs)  | 2.4               |
| [12] | [F#]          | [Day12.fsx](day-12/fsharp/Day12.fsx)                  | 5,834        | [day-12](day-12/rust/src/main.rs)  | 533               |
| [13] | [Swift]       | [13.swift](day-13/swift/13.swift)                     | 18.2         | [day-13](day-13/rust/src/main.rs)  | 2.0               |
| [14] | [C++]         | [14.cpp](day-14/cpp/14.cpp)                           | 21.6         | [day-14](day-14/rust/src/main.rs)  | 32.3              |
| [15] | [C]           | [15.c](day-15/c/15.c)                                 | 0.8          | [day-15](day-15/rust/src/main.rs)  | 1.4               |
| [16] | [Rune]        | [16.rn](day-16/rune/16.rn)                            | 5,440        | [day-16](day-16/rust/src/main.rs)  | 24                |
| [17] | [D]           | [app.d](day-17/d/source/app.d)                        | 578          | [day-17](day-17/rust/src/main.rs)  | 331               |
| [18] | [Moonscript]  | [18.moon](day-18/moonscript/18.moon)                  | 48.3         | [day-18](day-18/rust/src/main.rs)  | 1.8               |
| [19] | [Haskell]     | [19.hs](day-19/haskell/19.hs)                         | 17.0         | [day-19](day-19/rust/src/main.rs)  | 2.3               |
| [20] | [Scala]       | [Main.scala](day-20/scala/src/main/scala/Main.scala)  | 433          | [day-20](day-20/rust/src/main.rs)  | 16.2              |
| [21] | [Python]      | [21.py](day-21/python/21.py)                          | 690          | [day-21](day-21/rust/src/main.rs)  | 40.3              |
| [22] | [TypeScript]  | [22.ts](day-22/typescript/22.ts)                      | 876          | [day-22](day-22/rust/src/main.rs)  | 131               |
| [23] | —             | —                                                     | —            | [day-23](day-23/rust/src/main.rs)  | 4,500             |
| [24] | [Ruby]        | [24.rb](day-24/ruby/24.rb)                            | 160          | [day-24](day-24/rust/src/main.rs)  | 2.3               |
| [25] | [Go]          | [25.go](day-25/go/25.go)                              | 5,866        | [day-25](day-25/rust/src/main.rs)  | 4,234             |

All benchmarks produced with [hyperfine](https://github.com/sharkdp/hyperfine).

## What's all this?

[Advent of Code](https://adventofcode.com) is an annual "advent calendar"
event/challenge in which puzzles are released every day of December up to
Christmas Day in roughly increasing order of difficulty. This repository
represents my attempt to solve Advent of Code 2023 using a different language
each day (the "Polyglot Challenge", as I like to call it).

## What are the rules?

1. Every day must be solved in a different language. If I want to use a
   language that I used on a prior day, I first have to go back and solve that
   day in a different language.
2. Solutions must only use the standard library.
3. Have fun!

## How did you choose which languages to use?

Honestly, it's more madness than method. I have a big ol' document full of
languages I'd like to try. Other than front-loading weird languages for the
easier days and, conversely, saving languages I know well/are easy to learn for
the final stretch, I mostly picked languages randomly. There's a *bit* more
thought put into it than that (e.g., I avoided functional languages for
problems with multidimensional grids) but mostly I just relied on vibes.

## Should I do something like this?

Probably not. Maybe if you did it in the "off-season" it wouldn't be so bad but
trying to juggle nightly puzzles, learning a new language, and typical holiday
season obligations while still having sufficient time for sleep & work is a
tall order. Also, a few hours googling "how to do X in Y language" won't give
you much more than a surface level understanding of a language.

## Any highlights?

As always, Rust, Ruby, and OCaml stand out as my favorites. I definitely want
to spend more time with Crystal, Swift, Scala, and Clojure. Uiua is the most
approachable array language I've tried and I recommend giving it a spin if
you'd like to expand your ideas of how computation can be expressed.

## Any lowlights?

I learned that Lua is really not my jam and, by extension, anything built atop
it. I really wanted to like Fennel and Moonscript but Lua infects everything it
touches and the two suffer as a result.

## Did you make the leaderboard?

Nope but I placed 105<sup>th</sup> for day 23, part 2 which is the closest I've ever come!

## The quadratic formula is not a programming language!

I bet you're fun at parties.

[Uiua]: https://www.uiua.org/
[Fish shell]: https://fishshell.com/
[Crystal]: https://crystal-lang.org/
[Koka]: https://koka-lang.github.io/
[OCaml]: https://ocaml.org/
[Pen & Paper]: https://en.wikipedia.org/wiki/Quadratic_formula
[Clojure]: https://clojure.org/
[Inko]: https://inko-lang.org/
[SML]: https://en.wikipedia.org/wiki/Standard_ML
[Dart]: https://dart.dev/
[Fennel]: https://fennel-lang.org/
[F#]: https://fsharp.org/
[Swift]: https://www.swift.org/
[C++]: https://isocpp.org/
[C]: https://en.wikipedia.org/wiki/C_(programming_language)
[Rune]: https://rune-rs.github.io/
[D]: https://dlang.org/
[Moonscript]: https://moonscript.org/
[Haskell]: https://www.haskell.org/
[Scala]: https://www.scala-lang.org/
[Python]: https://www.python.org/
[TypeScript]: https://www.typescriptlang.org/
[Ruby]: https://www.ruby-lang.org/
[Go]: https://go.dev/

[1]: https://adventofcode.com/2023/day/1
[2]: https://adventofcode.com/2023/day/2
[3]: https://adventofcode.com/2023/day/3
[4]: https://adventofcode.com/2023/day/4
[5]: https://adventofcode.com/2023/day/5
[6]: https://adventofcode.com/2023/day/6
[7]: https://adventofcode.com/2023/day/7
[8]: https://adventofcode.com/2023/day/8
[9]: https://adventofcode.com/2023/day/9
[10]: https://adventofcode.com/2023/day/10
[11]: https://adventofcode.com/2023/day/11
[12]: https://adventofcode.com/2023/day/12
[13]: https://adventofcode.com/2023/day/13
[14]: https://adventofcode.com/2023/day/14
[15]: https://adventofcode.com/2023/day/15
[16]: https://adventofcode.com/2023/day/16
[17]: https://adventofcode.com/2023/day/17
[18]: https://adventofcode.com/2023/day/18
[19]: https://adventofcode.com/2023/day/19
[20]: https://adventofcode.com/2023/day/20
[21]: https://adventofcode.com/2023/day/21
[22]: https://adventofcode.com/2023/day/22
[23]: https://adventofcode.com/2023/day/23
[24]: https://adventofcode.com/2023/day/24
[25]: https://adventofcode.com/2023/day/25

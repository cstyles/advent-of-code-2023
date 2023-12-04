import std/os/file
import std/os/path

struct card
  number: int
  winning: list<int>
  have: list<int>

fun parse-card(line: string) : exn card
  val line = line.slice.advance(5) // trim "Card "
  val (number, line) = line.string.split-once(": ").unjust
  val number = number.parse-int.unjust

  val (winning, have) = line.split-once(" | ").unjust
  val winning = winning.split-whitespace.filter-map fn(s) s.parse-int
  val have = have.split-whitespace.filter-map fn(s) s.parse-int

  Card(number, winning, have)

fun card-value(card : card) : int
  match card.winning-count
    0 -> 0
    x -> 2.pow(x - 1)

fun winning-count(card : card) : int
  card.have.filter( fn(h) card.winning.contains(h)).length

fun main()
  // val input = "../test_input.txt".path.read-text-file.trim
  val input = "../input.txt".path.read-text-file.trim
  val cards = input.lines.map(parse-card)

  cards.map(card-value).sum.println // part1

fun split-once(s : string, sep : string) : maybe<(string, string)>
  match s.split(sep, 2)
    Cons(a, tail) -> match tail
      Cons(b, _) -> Just((a, b))
      _ -> Nothing
    _ -> Nothing

// Split on spaces, discarding any empty entries for multiple spaces.
fun split-whitespace(string : string) : list<string>
  string.split(" ").filter fn(s) !s.is-empty

fun contains(haystack : list<int>, needle : int) : bool
 haystack.any fn(item) item == needle

# Load input from file.
&fras "../input.txt"

# Split by newline.
⊜□≠@\n.

# 1-9 as chars.
CharDigits ← +@0+1⇡9

# Extract ASCII digits from a string and convert to integers.
ExtractDigits ← -@0▽∊,CharDigits

# Return the first and last items of a list, combined into a calibration value.
FirstAndLast ← +×10∩⊢:⇌.

# Get the calibration value for each line and sum them all up.
PartOne ← /+≡(FirstAndLast⊐ExtractDigits)

# Patterns to match against when searching for numbers.
Patterns ← {
  "1"
  "2"
  "3"
  "4"
  "5"
  "6"
  "7"
  "8"
  "9"
  "one"
  "two"
  "three"
  "four"
  "five"
  "six"
  "seven"
  "eight"
  "nine"
}

# Maps indices of patterns from `Patterns` to the values they should parse to.
ValueMap ← [1 2 3 4 5 6 7 8 9 1 2 3 4 5 6 7 8 9 0]

# Returns true if the top of the stack is a prefix for the second item.
PrefixMatches ← ≍⊙↙:⧻.

# Parse a number like "1" or "oneXX" to 1. Returns 0 if parsing failed.
ParseNumber ← ⊡:ValueMap⊗1⊞⊐PrefixMatches Patterns

# Extract numbers of both kinds and return the calibration value.
PartTwoFunc ← FirstAndLast ▽>0. ≡(ParseNumber□) ◫5⊂:"xxxx"⊔

# Get the calibration value for each line and sum them all up.
PartTwo ← /+≡PartTwoFunc

# Run both PartOne & PartTwo on the input.
⊃(PartTwo|PartOne)

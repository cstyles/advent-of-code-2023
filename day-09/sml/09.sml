fun is_space c = c = #" "
fun is_newline c = c = #"\n"
fun parse_num str = valOf (Int.fromString str)
fun parse_row row = map parse_num (String.tokens is_space row)
fun sum list = foldl op + 0 list
fun println str = print (str ^ "\n")

(* val file = TextIO.openIn "../test_input.txt" *)
val file = TextIO.openIn "../input.txt"
val input = TextIO.inputAll file
val lines = String.tokens is_newline input
val rows = map parse_row lines

(* Returns a list containing the first n items of a list. *)
fun take 0 _ = []
  | take _ [] = []
  | take n (h::t) = h :: take (n - 1) t

(* Returns a list of lists. Each sub-list is an N-item overlapping slice from the original list. *)
fun windows n list =
  case (n > length list, list) of
       (true, _) => []
     | (false, []) => []
     | (false, (h::t)) => (take n list) :: (windows n t)

(* Subtract the first item of a list from the second *)
fun sub2 (a :: [b]) = b - a
  | sub2 _ = raise Fail "bad windows"

fun solve row =
  let val next_row = map sub2 (windows 2 row)
  in
    if List.all (fn n => n = 0) next_row
    then List.last row
    else (List.last row) + solve next_row
  end

val part1 = sum (map solve rows)
val part2 = sum (map solve (map rev rows))

val _ = println ("part1 = " ^ (Int.toString part1))
val _ = println ("part2 = " ^ (Int.toString part2))

fun is_space c = c = #" "
fun is_newline c = c = #"\n"
fun parse_num str = Option.valOf (Int.fromString str)
fun parse_row row = List.map parse_num (String.fields is_space row)
fun trim str = Substring.string (Substring.trimr 1 (Substring.full str))
fun sum list = List.foldl Int.+ 0 list
fun println str = print (str ^ "\n")

(* val file = TextIO.openIn "../test_input.txt" *)
val file = TextIO.openIn "../input.txt"
val input = trim (TextIO.inputAll file)
val lines = String.fields is_newline input
val rows = List.map parse_row lines

(* Returns a list containing the first n items of a list. *)
fun take 0 _ = []
  | take _ [] = []
  | take n (h::t) = h :: take (n - 1) t

(* Returns a list of lists. Each sub-list is an N-item overlapping slice from the original list. *)
fun windows n list =
  case (n > List.length list, list) of
     (true, _) => []
     | (false, []) => []
     | (false, (h::t)) => (take n list) :: (windows n t)

(* Subtract the first item of a list from the second *)
fun sub2 (a :: [b]) = b - a
  | sub2 _ = raise Fail "bad windows"

fun solve row =
  let val next_row = List.map sub2 (windows 2 row)
  in
    if List.all (fn n => n = 0) next_row
    then List.last row
    else (List.last row) + solve next_row
  end

val part1 = sum (List.map solve rows)
val _ = println ("part1 = " ^ (Int.toString part1))

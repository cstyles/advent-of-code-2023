let input = open_in "../input.txt" |> In_channel.input_all |> String.trim
let lines = String.split_on_char '\n' input

(* Returns the last item of a list. *)
let rec last = function
  | [] -> None
  | x :: [] -> Some x
  | _ :: rest -> last rest

(* Returns the first and last items of a list. *)
let first_and_last = function
  | [] -> None
  | head :: tail -> Some (head, Option.value (last tail) ~default:head)

(* Turns a String into a char list. *)
let explode string = List.init (String.length string) (String.get string)

(* Parses a char as an int. *)
let parse_digit c =
  let code = Char.code c - Char.code '0' in
  if code < 10 then Some code else None

(* Function composition. *)
let ( % ) f g x = f (g x)

(* Extracts the first and last numeral digits from a String. *)
let extract_digits string =
  explode string |> List.filter_map parse_digit |> first_and_last |> Option.get

(* Converts a pair of numbers to a calibration value. *)
let calibration_value (first, last) = (first * 10) + last

(* Print an int followed by a newline. *)
let print_int_endline = print_endline % string_of_int

(* You can figure this one out. *)
let sum = List.fold_left ( + ) 0
let part1 = List.map (calibration_value % extract_digits) lines |> sum;;

print_int_endline part1

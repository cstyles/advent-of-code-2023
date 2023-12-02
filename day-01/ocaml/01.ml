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

(* Returns a new list containing the first n items of a list. *)
let rec take n list =
  match (n, list) with
  | 0, _ -> []
  | _, [] -> []
  | n, h :: t -> h :: take (n - 1) t

(* Returns a list of lists. Each sub-list is an N-item overlapping slice from the original list. *)
let rec windows n list =
  match (n > List.length list, list) with
  | true, _ -> []
  | false, [] -> []
  | false, _ :: t -> take n list :: windows n t

(* Parses a numeral digit or a number spelt out like "one". *)
let parse_number = function
  | '1' :: _ | 'o' :: 'n' :: 'e' :: _ -> Some 1
  | '2' :: _ | 't' :: 'w' :: 'o' :: _ -> Some 2
  | '3' :: _ | 't' :: 'h' :: 'r' :: 'e' :: 'e' :: _ -> Some 3
  | '4' :: _ | 'f' :: 'o' :: 'u' :: 'r' :: _ -> Some 4
  | '5' :: _ | 'f' :: 'i' :: 'v' :: 'e' :: _ -> Some 5
  | '6' :: _ | 's' :: 'i' :: 'x' :: _ -> Some 6
  | '7' :: _ | 's' :: 'e' :: 'v' :: 'e' :: 'n' :: _ -> Some 7
  | '8' :: _ | 'e' :: 'i' :: 'g' :: 'h' :: 't' :: _ -> Some 8
  | '9' :: _ | 'n' :: 'i' :: 'n' :: 'e' :: _ -> Some 9
  | _ -> None

let part2 =
  List.map (fun line -> line ^ "xxxx" |> explode |> windows 5) lines
  |> List.map (fun windows -> List.filter_map parse_number windows)
  |> List.map (Option.get % first_and_last)
  |> List.map calibration_value |> sum
;;

print_int_endline part2

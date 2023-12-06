#load "str.cma"

(* ==== Utilities ==== *)

(* Return a list without the first n items. *)
let rec skip n list =
  match (n, list) with
  | _, [] -> []
  | 0, list -> list
  | n, _ :: rest -> skip (n - 1) rest

(* Chunks a list into pairs of consecutive items. If the list contains an odd
   number of items, the last item will be dropped. *)
let rec pairs = function
  | [] -> []
  | [ _ ] -> []
  | x :: y :: rest -> (x, y) :: pairs rest

(* Slice string from pos until the end of the string. *)
let until_end string pos =
  let len = String.length string - pos in
  String.sub string pos len

let print_int_endline number = print_endline @@ string_of_int number
let min = List.fold_left Int.min Int.max_int

(* ==== Range type ==== *)

type range = { start : int; end_ : int }

let range_of_pair (start, length) = { start; end_ = start + length }
let range_contains number { start; end_ } = number >= start && number < end_

let debug_range { start; end_ } =
  print_endline "range:";
  print_endline (string_of_int start);
  print_endline (string_of_int end_)

(* ==== Mapping type ==== *)

type mapping = { source_start : int; dest_start : int; length : int }

let debug_mapping { source_start; dest_start; length } =
  print_endline "mapping:";
  print_endline (string_of_int source_start);
  print_endline (string_of_int dest_start);
  print_endline (string_of_int length)

let mapping_lookup number { source_start; dest_start; length } =
  number - source_start + dest_start

let mapping_source { source_start; dest_start; length } =
  { start = source_start; end_ = source_start + length }

(* ==== Parsing ==== *)

let parse_seeds line =
  until_end line 7 |> String.split_on_char ' ' |> List.map int_of_string

let parse_seed_ranges string =
  parse_seeds string |> pairs |> List.map range_of_pair

let parse_ranges line =
  match String.split_on_char ' ' line with
  | dest_start :: source_start :: [ length ] ->
      {
        source_start = int_of_string source_start;
        dest_start = int_of_string dest_start;
        length = int_of_string length;
      }
  | _ -> failwith "bad input (parse_ranges)"

let parse_map stuff =
  let lines = String.split_on_char '\n' stuff in
  let lines = skip 1 lines in
  List.map parse_ranges lines

(* let parse_maps = List.map parse_map *)
let parse_maps = function
  | [ a; b; c; d; e; f; g ] ->
      ( parse_map a,
        parse_map b,
        parse_map c,
        parse_map d,
        parse_map e,
        parse_map f,
        parse_map g )
  | _ -> failwith "bad maps"

let seeds, seed_ranges, maps =
  (* let input = String.trim (In_channel.input_all (open_in "../test_input.txt")) in *)
  let input = String.trim (In_channel.input_all (open_in "../input.txt")) in
  let sections = Str.split (Str.regexp "\n\n") input in
  match sections with
  | [] -> failwith "no input"
  | seeds :: rest ->
      (parse_seeds seeds, parse_seed_ranges seeds, parse_maps rest)

let lookup number mappings =
  List.find_opt
    (fun mapping -> range_contains number (mapping_source mapping))
    mappings
  |> Option.map (fun mapping -> mapping_lookup number mapping)

let ( seed_to_soil,
      soil_to_fertilizer,
      fertilizer_to_water,
      water_to_light,
      light_to_temperature,
      temperature_to_humidity,
      humidity_to_location ) =
  maps

let trace_through seed =
  let soil = Option.value (lookup seed seed_to_soil) ~default:seed in
  let fertilizer =
    Option.value (lookup soil soil_to_fertilizer) ~default:soil
  in
  let water =
    Option.value (lookup fertilizer fertilizer_to_water) ~default:fertilizer
  in
  let light = Option.value (lookup water water_to_light) ~default:water in
  let temperature =
    Option.value (lookup light light_to_temperature) ~default:light
  in
  let humidity =
    Option.value
      (lookup temperature temperature_to_humidity)
      ~default:temperature
  in
  let location =
    Option.value (lookup humidity humidity_to_location) ~default:humidity
  in
  location
;;

print_int_endline @@ min (List.map trace_through seeds)

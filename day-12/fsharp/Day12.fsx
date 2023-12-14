// For more information see https://aka.ms/fsharp-console-apps

open System.IO

type Status =
    | Operational
    | Damaged
    | Unknown

let parse_status =
    function
    | '.' -> Operational
    | '#' -> Damaged
    | '?' -> Unknown
    | c -> failwith $"bad input: {c}"

type Row =
    { springs: Status list
      groups: int list }

let parse_row (line: string) =
    let split = line.Split " " in
    let springs = split[0].ToCharArray() |> Array.map parse_status |> Array.toList in
    let groups = split[1].Split "," |> Array.map int |> Array.toList in
    { springs = springs; groups = groups }

let replace_head new_head =
    function
    | [] -> []
    | head :: tail -> new_head :: tail

// let input = "../test_input.txt"
let input = "../input.txt"

let lines = File.ReadLines input |> Seq.map (fun x -> x.TrimEnd())
let rows = Seq.map parse_row lines

let rec trim_and_continue (row: Row) =
    match row.springs with
    | head :: tail -> possible_arrangements { row with springs = tail }
    | _ -> failwith "empty row"

and process_stuff (row: Row) =
    match row.springs, row.groups with
    | [], [ 0 ] -> 1
    | [], 0 :: _ -> 0
    | [], _ -> 0
    | Damaged :: _rest, 0 :: _ -> 0
    | Damaged :: rest, group_size :: rest_of_groups ->
        process_stuff
            { springs = rest
              groups = (group_size - 1) :: rest_of_groups }
    | Operational :: rest_of_springs, 0 :: rest_of_groups ->
        possible_arrangements
            { springs = rest_of_springs
              groups = rest_of_groups }
    | Operational :: _, _ -> 0 // invalid
    | Unknown :: rest_of_springs, 0 :: rest_of_groups ->
        possible_arrangements
            { springs = rest_of_springs
              groups = rest_of_groups }
    | Unknown :: rest, group_size :: rest_of_groups ->
        process_stuff
            { springs = rest
              groups = (group_size - 1) :: rest_of_groups }
    | _, [] -> failwith "groups can't be empty"

and possible_arrangements row =
    match (List.tryHead row.springs, List.tryHead row.groups) with
    | None, None -> 1 // done
    | None, Some _ -> 0 // invalid
    | Some Damaged, None -> 0 // invalid
    | Some Unknown, None -> trim_and_continue row
    | Some Operational, _ -> trim_and_continue row
    | Some Damaged, Some _group_size -> process_stuff row
    | Some Unknown, Some _ ->
        let springs' = replace_head Operational row.springs in
        let new_row = { row with springs = springs' } in
        let try_operational = possible_arrangements new_row in
        let springs' = replace_head Damaged row.springs in
        let new_row = { row with springs = springs' } in
        let try_damaged = possible_arrangements new_row in
        try_operational + try_damaged

Seq.map possible_arrangements rows |> Seq.sum |> printfn "part1 = %d"
